use crate::ui::markdown::render_markdown;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Text,
    widgets::{Block, Borders, Paragraph, Wrap},
};
use std::sync::OnceLock;

pub fn draw_ui(
    frame: &mut Frame,
    monitor_text: &str,
    status_text: &str,
    response_text: &str,
    scroll: u16,
) {
    let area = frame.area();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(1)])
        .split(area);

    let header_title = format!(" AI Watcher | Monitor: {} ", monitor_text);
    let header_block = Block::default()
        .title(header_title)
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Cyan));

    let header = Paragraph::new(status_text).block(header_block).style(
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD),
    );

    frame.render_widget(header, chunks[0]);

    static LAST_RESPONSE: OnceLock<std::sync::Mutex<(String, Text<'static>)>> = OnceLock::new();

    let cache =
        LAST_RESPONSE.get_or_init(|| std::sync::Mutex::new((String::new(), Text::default())));

    let rendered_response = {
        let mut cache = cache.lock().unwrap();
        if cache.0 != response_text {
            cache.1 = render_markdown(response_text);
            cache.0 = response_text.to_string();
        }
        cache.1.clone()
    };

    let content_block = Block::default()
        .title(" AI Suggestion | ↑/↓ or j/k to scroll | q to quit ")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Green));

    let content = Paragraph::new(rendered_response)
        .block(content_block)
        .wrap(Wrap { trim: false })
        .scroll((scroll, 0));

    frame.render_widget(content, chunks[1]);
}
