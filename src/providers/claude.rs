use super::{ProviderError, ProviderTrait, BaseProvider};
use serde_json::{json, Value};
use tokio::sync::mpsc;
use std::fmt;
use std::sync::{Arc, Mutex};
use futures_util::StreamExt;
use crate::app::ProfileType;

pub struct Claude {
    base: Arc<Mutex<BaseProvider>>,
    current_model: Arc<Mutex<String>>,
}

impl Claude {
    pub fn new(api_key: String) -> Self {
        Self {
            base: Arc::new(Mutex::new(BaseProvider::new(api_key))),
            current_model: Arc::new(Mutex::new("claude-3-5-sonnet-20240620".to_string())),
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
            "Other",
        ]
    }

    fn stream_response(&self, messages: Vec<Value>) -> Result<mpsc::Receiver<String>, ProviderError> {
        let model = self.current_model.lock().unwrap().clone();
        let (top_p, top_k, repetition_penalty, creativity) = self.base.lock().unwrap().get_parameters();
        let client = self.base.lock().unwrap().get_client();
        let api_key = self.base.lock().unwrap().get_api_key();

        let json_body = json!({
            "model": model,
            "messages": messages,
            "max_tokens": 8192,
            "stream": true,
            "temperature": creativity,
            "top_p": top_p,
            "top_k": top_k,
        });

        println!("Debug: Model parameters - top_p: {}, top_k: {}, repetition_penalty: {}, creativity: {}", top_p, top_k, repetition_penalty, creativity);

        let (tx, rx) = mpsc::channel(1024);
        
        tokio::task::spawn(async move {
            let response = client
                .post("https://api.anthropic.com/v1/messages")
                .header("x-api-key", api_key)
                .header("anthropic-version", "2023-06-01")
                .header("content-type", "application/json")
                .json(&json_body)
                .send()
                .await;

            match response {
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
                                buffer.push_str(&String::from_utf8_lossy(&chunk));

                                while let Some(newline_pos) = buffer.find('\n') {
                                    let line = buffer[..newline_pos].trim().to_string();
                                    buffer = buffer[newline_pos + 1..].to_string();

                                    if line.starts_with("data: ") {
                                        let data = &line["data: ".len()..];
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

    fn set_current_model(&self, model: String) {
        *self.current_model.lock().unwrap() = model;
    }

    fn update_profile(&self, profile: ProfileType) {
        self.base.lock().unwrap().update_profile(profile);
    }

    fn get_parameters(&self) -> (f32, u32, f32, f32) {
        self.base.lock().unwrap().get_parameters()
    }
}

impl fmt::Display for Claude {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Claude")
    }
}