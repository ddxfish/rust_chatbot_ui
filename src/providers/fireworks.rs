use super::{Provider, ProviderError};
use std::fmt;
use reqwest::Client;
use serde_json::{json, Value};
use tokio::sync::mpsc;
use futures_util::StreamExt;

pub struct Fireworks {
    client: Client,
    api_key: String,
}

impl Fireworks {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
        }
    }
}

#[async_trait::async_trait]
impl Provider for Fireworks {
    fn name(&self) -> &'static str {
        "Fireworks"
    }

    fn models(&self) -> Vec<&'static str> {
        vec![
            "llama-v3p1-405b-instruct",
            "llama-v3p1-70b-instruct",
            "llama-v3p1-8b-instruct",
            "Other",
        ]
    }

    async fn stream_response(&self, messages: Vec<Value>) -> Result<mpsc::Receiver<String>, ProviderError> {
        let (tx, rx) = mpsc::channel(1024);
        let client = self.client.clone();
        let api_key = self.api_key.clone();

        tokio::spawn(async move {
            let response = match client
                .post("https://api.fireworks.ai/inference/v1/chat/completions")
                .header("Accept", "application/json")
                .header("Content-Type", "application/json")
                .header("Authorization", format!("Bearer {}", api_key))
                .json(&json!({
                    "model": "accounts/fireworks/models/llama-v3p1-70b-instruct",
                    "max_tokens": 16384,
                    "top_p": 1,
                    "top_k": 40,
                    "presence_penalty": 0,
                    "frequency_penalty": 0,
                    "temperature": 0.6,
                    "messages": messages,
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
                                        println!("Debug: Stream completed");
                                        return;
                                    }

                                    match serde_json::from_str::<Value>(data) {
                                        Ok(json) => {
                                            if let Some(content) = json["choices"][0]["delta"]["content"].as_str() {
                                                println!("Debug: Received content chunk: {}", content);
                                                if tx.send(content.to_string()).await.is_err() {
                                                    eprintln!("Error sending chunk through channel");
                                                    return;
                                                }
                                            }
                                        }
                                        Err(e) => eprintln!("Error parsing JSON: {:?}", e),
                                    }
                                }
                            }
                        } else {
                            eprintln!("Error converting chunk to UTF-8");
                        }
                    }
                    Err(e) => {
                        eprintln!("Error receiving chunk: {:?}", e);
                    }
                }
            }

            if !buffer.is_empty() {
                eprintln!("Unprocessed data in buffer: {}", buffer);
            }
        });

        Ok(rx)
    }
}

impl fmt::Display for Fireworks {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Fireworks")
    }
}