use log::{debug, error};
use serde_json::json;

pub async fn send_message_to_api(conversation_history: Vec<serde_json::Value>, api_key: String) -> Result<String, String> {
    debug!("Sending message to API");
    let client = reqwest::Client::new();
    let response = client
        .post("https://api.fireworks.ai/inference/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&json!({
            "model": "accounts/fireworks/models/llama-v3-8b-instruct",
            "messages": conversation_history
        }))
        .send()
        .await
        .map_err(|e| {
            error!("API request error: {}", e);
            e.to_string()
        })?;

    let response_json: serde_json::Value = response.json().await.map_err(|e| {
        error!("Failed to parse API response: {}", e);
        e.to_string()
    })?;
    let bot_message = response_json["choices"][0]["message"]["content"]
        .as_str()
        .ok_or("Failed to parse response")?
        .to_string();

    debug!("Received bot message: {}", bot_message);
    Ok(bot_message)
}