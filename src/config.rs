use crate::error::{AppError, Result};

use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct AppConfig {
    pub openai_api_key: String,
    pub trigger_keywords: Vec<String>,
    pub ai_system_prompt: String,
    pub ai_user_prompt: String,
}

impl AppConfig {
    pub fn load() -> Result<Self> {
        let _ = dotenvy::dotenv();
        Self::from_env(|key| std::env::var(key).ok())
    }

    pub fn from_env<F>(mut env_getter: F) -> Result<Self>
    where
        F: FnMut(&str) -> Option<String>,
    {
        let openai_api_key = env_getter("OPENAI_API_KEY").unwrap_or_default();

        if openai_api_key.is_empty() {
            return Err(AppError::Config(
                "OPENAI_API_KEY is missing from environment".into(),
            ));
        }

        let raw_keywords =
            env_getter("TRIGGER_KEYWORDS").unwrap_or_else(|| "hello, world".to_string());

        let trigger_keywords = raw_keywords
            .split(',')
            .map(|s| s.trim().to_lowercase())
            .filter(|s| !s.is_empty())
            .collect();

        let ai_system_prompt = env_getter("AI_SYSTEM_PROMPT").unwrap_or_else(|| {
            "You are an expert coding assistant. Solve the problem efficiently.".to_string()
        });

        let ai_user_prompt = env_getter("AI_USER_PROMPT").unwrap_or_else(|| {
            "Solve this task cleanly and efficiently. Return Markdown with fenced code blocks and language identifiers when appropriate.".to_string()
        });

        Ok(Self {
            openai_api_key,
            trigger_keywords,
            ai_system_prompt,
            ai_user_prompt,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_missing_api_key() {
        let result = AppConfig::from_env(|_| None);
        assert!(result.is_err());
    }

    #[test]
    fn test_config_load_success() {
        let config = AppConfig::from_env(|key| match key {
            "OPENAI_API_KEY" => Some("sk-test-12345".to_string()),
            "TRIGGER_KEYWORDS" => Some("  rust , cargo , test ".to_string()),
            "AI_SYSTEM_PROMPT" => Some("Custom prompt".to_string()),
            "AI_USER_PROMPT" => Some("Custom user prompt".to_string()),
            _ => None,
        })
        .unwrap();

        assert_eq!(config.openai_api_key, "sk-test-12345");
        assert_eq!(config.trigger_keywords, vec!["rust", "cargo", "test"]);
        assert_eq!(config.ai_system_prompt, "Custom prompt");
        assert_eq!(config.ai_user_prompt, "Custom user prompt");
    }

    #[test]
    fn test_config_defaults() {
        let config = AppConfig::from_env(|key| match key {
            "OPENAI_API_KEY" => Some("sk-test-12345".to_string()),
            _ => None,
        })
        .unwrap();

        assert_eq!(config.openai_api_key, "sk-test-12345");
        assert_eq!(config.trigger_keywords, vec!["hello", "world"]);
        assert_eq!(
            config.ai_system_prompt,
            "You are an expert coding assistant. Solve the problem efficiently."
        );
        assert_eq!(
            config.ai_user_prompt,
            "Solve this task cleanly and efficiently. Return Markdown with fenced code blocks and language identifiers when appropriate."
        );
    }
}
