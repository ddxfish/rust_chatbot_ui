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
        ]
    }

    async fn generate_response(&self, messages: Vec<Value>) -> Result<String, ProviderError> {
        let response = self.client
            .post("https://api.fireworks.ai/inference/v1/chat/completions")
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&json!({
                "model": "accounts/fireworks/models/llama-v3p1-70b-instruct",
                "max_tokens": 16384,
                "top_p": 1,
                "top_k": 40,
                "presence_penalty": 0,
                "frequency_penalty": 0,
                "temperature": 0.6,
                "messages": messages
            }))
            .send()
            .await
            .map_err(|e| ProviderError::RequestError(e.to_string()))?;

        let response_json: Value = response.json().await
            .map_err(|e| ProviderError::ResponseError(e.to_string()))?;
        
        Ok(response_json["choices"][0]["message"]["content"]
            .as_str()
            .ok_or_else(|| ProviderError::ResponseError("Invalid response format".to_string()))?
            .to_string())
    }

    async fn stream_response(&self, messages: Vec<Value>) -> Result<mpsc::Receiver<String>, ProviderError> {
        let (tx, rx) = mpsc::channel(100);
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
            while let Some(item) = stream.next().await {
                match item {
                    Ok(chunk) => {
                        if let Ok(text) = String::from_utf8(chunk.to_vec()) {
                            for line in text.lines() {
                                if line.starts_with("data: ") {
                                    let data = &line[6..];
                                    if data != "[DONE]" {
                                        if let Ok(json) = serde_json::from_str::<Value>(data) {
                                            if let Some(content) = json["choices"][0]["delta"]["content"].as_str() {
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
                    },
                    Err(e) => {
                        eprintln!("Error reading stream: {:?}", e);
                        return;
                    }
                }
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