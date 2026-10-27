use crate::{
    ai::{client::query_vision, parser::parse_openai_response},
    config::AppConfig,
    constants::{
        DEFAULT_FALLBACK_MONITOR, POLL_INTERVAL_MS, SCREENSHOT_PATH, TRIGGER_COOLDOWN_SECS,
        WATCHER_SLEEP_SECS,
    },
    error::Result,
    models::state::AppState,
    ui::renderer::draw_ui,
    utils::{
        hyprland::get_active_monitor_name,
        system::{capture_screen, run_ocr},
    },
};
use crossterm::{
    ExecutableCommand,
    event::{self, Event, KeyCode},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::{
    fs, io,
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::time::sleep;

pub async fn background_watcher_loop(state: Arc<Mutex<AppState>>, config: Arc<AppConfig>) {
    loop {
        let monitor_name =
            get_active_monitor_name().unwrap_or_else(|| DEFAULT_FALLBACK_MONITOR.to_string());

        {
            let mut s = state.lock().unwrap();
            s.active_monitor = monitor_name.clone();
            s.status = format!("Observing [{}] | Looking for keywords...", monitor_name);
        }

        if let Err(err) = capture_screen(&monitor_name, SCREENSHOT_PATH) {
            state.lock().unwrap().status = format!("Screenshot error: {}", err);
            sleep(Duration::from_secs(5)).await;
            continue;
        }

        match run_ocr(SCREENSHOT_PATH) {
            Ok(extracted_text) => {
                let matched_keyword = config
                    .trigger_keywords
                    .iter()
                    .find(|kw| extracted_text.contains(kw.as_str()));

                if let Some(keyword) = matched_keyword {
                    {
                        let mut s = state.lock().unwrap();
                        s.status = format!("Keyword '{}' matched! Querying OpenAI...", keyword);
                    }

                    if let Ok(image_bytes) = fs::read(SCREENSHOT_PATH) {
                        match query_vision(&image_bytes, &config).await {
                            Ok(response) => {
                                let clean_solution = parse_openai_response(&response);
                                let mut s = state.lock().unwrap();
                                s.ai_response = clean_solution;
                                s.status =
                                    "Solution generated! Use Up/Down or j/k to scroll.".to_string();
                                s.scroll_offset = 0;
                            }
                            Err(err) => {
                                state.lock().unwrap().status = format!("API Error: {}", err);
                            }
                        }
                    }

                    sleep(Duration::from_secs(TRIGGER_COOLDOWN_SECS)).await;
                }
            }
            Err(err) => {
                state.lock().unwrap().status = format!("OCR error: {}", err);
            }
        }

        sleep(Duration::from_secs(WATCHER_SLEEP_SECS)).await;
    }
}

pub async fn run_app(state: Arc<Mutex<AppState>>) -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    stdout.execute(EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_tui_loop(&mut terminal, state).await;

    // Guaranteed cleanup on exit
    disable_raw_mode()?;
    terminal.backend_mut().execute(LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    res
}

async fn run_tui_loop(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    state: Arc<Mutex<AppState>>,
) -> Result<()> {
    loop {
        let (status_text, monitor_text, response_text, scroll) = {
            let s = state.lock().unwrap();
            (
                s.status.clone(),
                s.active_monitor.clone(),
                s.ai_response.clone(),
                s.scroll_offset,
            )
        };

        terminal.draw(|frame| {
            draw_ui(frame, &monitor_text, &status_text, &response_text, scroll);
        })?;

        if event::poll(Duration::from_millis(POLL_INTERVAL_MS))? {
            let Event::Key(key) = event::read()? else {
                continue;
            };

            let mut s = state.lock().unwrap();
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => break,
                KeyCode::Char('j') | KeyCode::Down => {
                    s.scroll_offset = s.scroll_offset.saturating_add(1);
                }
                KeyCode::Char('k') | KeyCode::Up => {
                    s.scroll_offset = s.scroll_offset.saturating_sub(1);
                }
                KeyCode::PageDown => {
                    s.scroll_offset = s.scroll_offset.saturating_add(10);
                }
                KeyCode::PageUp => {
                    s.scroll_offset = s.scroll_offset.saturating_sub(10);
                }
                _ => {}
            }
        }
    }

    Ok(())
}
