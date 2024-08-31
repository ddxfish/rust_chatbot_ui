use super::{Provider, ProviderError};
use std::fmt;
use reqwest::Client;
use serde_json::{json, Value};
use tokio::sync::mpsc;
use futures_util::StreamExt;

pub struct Claude {
    client: Client,
    api_key: String,
}

impl Claude {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
        }
    }
}

#[async_trait::async_trait]
impl Provider for Claude {
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

    async fn stream_response(&self, messages: Vec<Value>) -> Result<mpsc::Receiver<String>, ProviderError> {
        let (tx, rx) = mpsc::channel(1024);
        let client = self.client.clone();
        let api_key = self.api_key.clone();

        tokio::spawn(async move {
            let response = match client
                .post("https://api.anthropic.com/v1/messages")
                .header("anthropic-version", "2023-06-01")
                .header("content-type", "application/json")
                .header("x-api-key", api_key)
                .json(&json!({
                    "model": "claude-3-5-sonnet-20240620",
                    "messages": messages,
                    "max_tokens": 1024,
                    "stream": true
                }))
                .send()
                .await {
                    Ok(res) => res,
                    Err(e) => {
                        eprintln!("Error sending request to Claude API: {:?}", e);
                        let _ = tx.send(format!("Error: Failed to send request to Claude API - {}", e)).await;
                        return;
                    }
                };

            if !response.status().is_success() {
                let error_body = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
                eprintln!("Error response from Claude API: {}", error_body);
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

                                    match serde_json::from_str::<Value>(data) {
                                        Ok(json) => {
                                            if let Some(content) = json["delta"]["text"].as_str() {
                                                if tx.send(content.to_string()).await.is_err() {
                                                    eprintln!("Error sending chunk through channel");
                                                    return;
                                                }
                                            }
                                        }
                                        Err(e) => {
                                            eprintln!("Error parsing JSON from Claude: {:?}", e);
                                            let _ = tx.send(format!("Error: Failed to parse Claude response - {}", e)).await;
                                        }
                                    }
                                }
                            }
                        } else {
                            eprintln!("Error converting Claude response chunk to UTF-8");
                            let _ = tx.send("Error: Failed to convert Claude response to text".to_string()).await;
                        }
                    }
                    Err(e) => {
                        eprintln!("Error receiving chunk from Claude: {:?}", e);
                        let _ = tx.send(format!("Error: Failed to receive response from Claude - {}", e)).await;
                        return;
                    }
                }
            }

            if !buffer.is_empty() {
                eprintln!("Unprocessed data in buffer from Claude: {}", buffer);
                let _ = tx.send(format!("Error: Unprocessed data from Claude - {}", buffer)).await;
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