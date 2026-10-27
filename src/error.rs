use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Network request error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("JSON serialization/deserialization error: {0}")]
    SerdeJson(#[from] serde_json::Error),
    #[error("Environment configuration missing: {0}")]
    Config(String),
    #[error("System command execution error: {0}")]
    Command(String),
}

pub type Result<T> = std::result::Result<T, AppError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_error_display() {
        let err_config = AppError::Config("API key missing".to_string());
        assert_eq!(
            format!("{}", err_config),
            "Environment configuration missing: API key missing"
        );

        let err_cmd = AppError::Command("grim failed".to_string());
        assert_eq!(
            format!("{}", err_cmd),
            "System command execution error: grim failed"
        );

        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let err_io = AppError::from(io_err);
        assert!(format!("{}", err_io).contains("IO error"));
    }
}
