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
        let (tx, rx) = mpsc::channel(100);
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
                        eprintln!("Error sending request: {:?}", e);
                        return;
                    }
                };

            let mut stream = response.bytes_stream();
            while let Some(item) = stream.next().await {
                if let Ok(chunk) = item {
                    if let Ok(text) = String::from_utf8(chunk.to_vec()) {
                        for line in text.lines() {
                            if line.starts_with("data: ") {
                                let data = &line[6..];
                                if data != "[DONE]" {
                                    if let Ok(json) = serde_json::from_str::<Value>(data) {
                                        if let Some(content) = json["delta"]["text"].as_str() {
                                            if tx.send(content.to_string()).await.is_err() {
                                                eprintln!("Error sending chunk through channel");
                                                return;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
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