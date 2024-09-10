use super::{ProviderError, ProviderTrait, BaseProvider};
use serde_json::{json, Value};
use tokio::sync::mpsc;
use std::fmt;
use std::sync::Arc;
use futures_util::StreamExt;  // Add this import
pub struct Claude {
    base: Arc<BaseProvider>,
}

impl Claude {
    pub fn new(api_key: String) -> Self {
        Self {
            base: Arc::new(BaseProvider::new(api_key)),
        }
    }
}

impl ProviderTrait for Claude {
    fn name(&self) -> &'static str {
        "Claude"
    }

    fn models(&self) -> Vec<&'static str> {
        vec![
            "claude-3-5-sonnet-20240620",
            "claude-3-opus-20240229",
            "claude-3-haiku-20240307",
        ]
    }

    fn stream_response(&self, messages: Vec<Value>) -> Result<mpsc::Receiver<String>, ProviderError> {
        let json_body = json!({
            "model": "claude-3-5-sonnet-20240620",
            "messages": messages,
            "max_tokens": 1024,
            "stream": true
        });

        let (tx, rx) = mpsc::channel(1024);
        let base = self.base.clone();
        
        tokio::spawn(async move {
            match base.client
                .post("https://api.anthropic.com/v1/messages")
                .header("x-api-key", &base.api_key)
                .header("anthropic-version", "2023-06-01")
                .header("content-type", "application/json")
                .json(&json_body)
                .send()
                .await {
                Ok(response) => {
                    if !response.status().is_success() {
                        let error_body = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
                        let _ = tx.send(format!("Error: Claude API returned error - {}", error_body)).await;
                        return;
                    }

                    let mut stream = response.bytes_stream();
                    let mut buffer = String::new();

                    while let Some(item) = stream.next().await {
                        match item {
                            Ok(chunk) => {
                                if let Ok(text) = String::from_utf8(chunk.to_vec()) {
                                    buffer.push_str(&text);

                                    while let Some(pos) = buffer.find('\n') {
                                        let line = buffer[..pos].to_string();
                                        buffer = buffer[pos + 1..].to_string();

                                        if line.starts_with("data: ") {
                                            let data = &line[6..];
                                            if data == "[DONE]" {
                                                return;
                                            }

                                            if let Ok(json) = serde_json::from_str::<Value>(data) {
                                                if let Some(content) = json["delta"]["text"].as_str() {
                                                    if tx.send(content.to_string()).await.is_err() {
                                                        return;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            Err(e) => {
                                let _ = tx.send(format!("Error: Failed to receive response from Claude - {}", e)).await;
                                return;
                            }
                        }
                    }
                }
                Err(e) => {
                    let _ = tx.send(format!("Error: Failed to send request to Claude API - {}", e)).await;
                }
            }
        });

        Ok(rx)
    }
}

impl fmt::Display for Claude {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Claude")
    }
}