use super::{Provider, ProviderError};
use std::fmt;
use reqwest::Client;
use serde_json::{json, Value};
use tokio::sync::mpsc;
use futures_util::StreamExt;

pub struct GPT {
    client: Client,
    api_key: String,
}

impl GPT {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
        }
    }
}

#[async_trait::async_trait]
impl Provider for GPT {
    fn name(&self) -> &'static str {
        "GPT"
    }

    fn models(&self) -> Vec<&'static str> {
        vec![
            "gpt-4",
            "gpt-4-32k",
            "gpt-3.5-turbo",
            "gpt-3.5-turbo-16k",
        ]
    }

    async fn stream_response(&self, messages: Vec<Value>) -> Result<mpsc::Receiver<String>, ProviderError> {
        let (tx, rx) = mpsc::channel(1024);
        let client = self.client.clone();
        let api_key = self.api_key.clone();

        tokio::spawn(async move {
            let response = match client
                .post("https://api.openai.com/v1/chat/completions")
                .header("Authorization", format!("Bearer {}", api_key))
                .header("Content-Type", "application/json")
                .json(&json!({
                    "model": "gpt-3.5-turbo",
                    "messages": messages,
                    "stream": true
                }))
                .send()
                .await {
                    Ok(res) => res,
                    Err(e) => {
                        eprintln!("Error sending request to GPT API: {:?}", e);
                        let _ = tx.send(format!("Error: Failed to send request to GPT API - {}", e)).await;
                        return;
                    }
                };

            if !response.status().is_success() {
                let error_body = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
                eprintln!("Error response from GPT API: {}", error_body);
                let _ = tx.send(format!("Error: GPT API returned error - {}", error_body)).await;
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
                                            if let Some(content) = json["choices"][0]["delta"]["content"].as_str() {
                                                if tx.send(content.to_string()).await.is_err() {
                                                    eprintln!("Error sending chunk through channel");
                                                    return;
                                                }
                                            }
                                        }
                                        Err(e) => eprintln!("Error parsing JSON from GPT: {:?}", e),
                                    }
                                }
                            }
                        } else {
                            eprintln!("Error converting GPT response chunk to UTF-8");
                        }
                    }
                    Err(e) => {
                        eprintln!("Error receiving chunk from GPT: {:?}", e);
                        let _ = tx.send(format!("Error: Failed to receive response from GPT - {}", e)).await;
                        return;
                    }
                }
            }

            if !buffer.is_empty() {
                eprintln!("Unprocessed data in buffer from GPT: {}", buffer);
                let _ = tx.send(format!("Error: Unprocessed data from GPT - {}", buffer)).await;
            }
        });

        Ok(rx)
    }
}

impl fmt::Display for GPT {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "GPT")
    }
}