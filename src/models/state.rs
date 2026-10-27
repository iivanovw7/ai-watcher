#[derive(Debug, Clone)]
pub struct AppState {
    pub status: String,
    pub active_monitor: String,
    pub ai_response: String,
    pub scroll_offset: u16,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            status: "Initializing background watcher...".to_string(),
            active_monitor: "Scanning...".to_string(),
            ai_response: "Waiting for trigger keyword...".to_string(),
            scroll_offset: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_state_new() {
        let state = AppState::new();
        assert_eq!(state.status, "Initializing background watcher...");
        assert_eq!(state.active_monitor, "Scanning...");
        assert_eq!(state.ai_response, "Waiting for trigger keyword...");
        assert_eq!(state.scroll_offset, 0);
    }
}
