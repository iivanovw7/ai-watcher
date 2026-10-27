use crate::config::AppConfig;
use crate::error::Result;

pub async fn query_vision(image_bytes: &[u8], config: &AppConfig) -> Result<String> {
    let client = reqwest::Client::new();
    let base64_image =
        base64::Engine::encode(&base64::engine::general_purpose::STANDARD, image_bytes);

    let payload = serde_json::json!({
        "model": "gpt-4o",
        "messages": [
            {
                "role": "system",
                "content": &config.ai_system_prompt
            },
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": &config.ai_user_prompt
                    },
                    {
                        "type": "image_url",
                        "image_url": {
                            "url": format!("data:image/png;base64,{}", base64_image)
                        }
                    }
                ]
            }
        ],
        "max_tokens": 1500
    });

    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", config.openai_api_key))
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await?
        .error_for_status()?;

    Ok(response.text().await?)
}
