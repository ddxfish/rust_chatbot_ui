use crate::providers::ProviderError;
use reqwest::Client;
use serde_json::Value;
use tokio::sync::mpsc;
use std::fmt;
use futures_util::StreamExt;

pub struct BaseProvider {
    pub client: Client,
    pub api_key: String,
}

impl BaseProvider {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
        }
    }

    pub async fn stream_response(&self, url: &str, json_body: Value, error_prefix: &str) -> Result<mpsc::Receiver<String>, ProviderError> {
        let (tx, rx) = mpsc::channel(1024);
        
        println!("Debug: Accessing URL: {}", url);
        
        let response = self.client
            .post(url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&json_body)
            .send()
            .await
            .map_err(|e| ProviderError::RequestError(format!("{}: {}", error_prefix, e)))?;

        if !response.status().is_success() {
            let error_body = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(ProviderError::ResponseError(format!("{}: {}", error_prefix, error_body)));
        }

        let mut stream = response.bytes_stream();
        
        tokio::spawn(async move {
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
                                        if let Some(content) = json["choices"][0]["delta"]["content"].as_str() {
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
                        let _ = tx.send(format!("Error: Failed to receive response - {}", e)).await;
                        return;
                    }
                }
            }
        });

        Ok(rx)
    }
}

pub trait ProviderTrait: fmt::Display + Send + Sync {
    fn name(&self) -> &'static str;
    fn models(&self) -> Vec<&'static str>;
    fn stream_response(&self, messages: Vec<Value>) -> Result<mpsc::Receiver<String>, ProviderError>;
    fn set_current_model(&self, model: String);
}