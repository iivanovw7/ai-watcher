pub fn parse_openai_response(json_str: &str) -> String {
    if let Ok(value) = serde_json::from_str::<serde_json::Value>(json_str) {
        if let Some(content) = value["choices"]
            .get(0)
            .and_then(|choice| choice["message"]["content"].as_str())
        {
            return content.trim().to_string();
        }

        if let Some(error) = value["error"]["message"].as_str() {
            return format!("OpenAI API error: {}", error);
        }
    }

    json_str.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_openai_response_success() {
        let json = r#"{
            "choices": [
                {
                    "message": {
                        "content": "  Hello from OpenAI!   \n"
                    }
                }
            ]
        }"#;
        assert_eq!(parse_openai_response(json), "Hello from OpenAI!");
    }

    #[test]
    fn test_parse_openai_response_error() {
        let json = r#"{
            "error": {
                "message": "Rate limit exceeded"
            }
        }"#;
        assert_eq!(
            parse_openai_response(json),
            "OpenAI API error: Rate limit exceeded"
        );
    }

    #[test]
    fn test_parse_openai_response_fallback() {
        let raw = "Plain text response without json";
        assert_eq!(
            parse_openai_response(raw),
            "Plain text response without json"
        );

        let invalid_json = "{ invalid json }";
        assert_eq!(parse_openai_response(invalid_json), "{ invalid json }");
    }
}
