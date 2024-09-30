use super::{ProviderError, ProviderTrait, BaseProvider};
use serde_json::{json, Value};
use tokio::sync::mpsc;
use std::fmt;
use std::sync::{Arc, Mutex};
use crate::app::ProfileType;
use futures_util::StreamExt;

pub struct Fireworks {
    base: Arc<Mutex<BaseProvider>>,
    current_model: Arc<Mutex<String>>,
}

impl Fireworks {
    pub fn new(api_key: String) -> Self {
        Self {
            base: Arc::new(Mutex::new(BaseProvider::new(api_key))),
            current_model: Arc::new(Mutex::new("accounts/fireworks/models/llama-v3p1-70b-instruct".to_string())),
        }
    }
}

impl ProviderTrait for Fireworks {
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

    fn stream_response(&self, messages: Vec<Value>) -> Result<mpsc::Receiver<String>, ProviderError> {
        let model = self.current_model.lock().unwrap().clone();
        let (top_p, top_k, repetition_penalty, creativity) = self.base.lock().unwrap().get_parameters();
        let client = self.base.lock().unwrap().get_client();
        let api_key = self.base.lock().unwrap().get_api_key();

        let json_body = json!({
            "model": model,
            "max_tokens": 16384,
            "top_p": top_p,
            "top_k": top_k,
            "presence_penalty": 0,
            "frequency_penalty": repetition_penalty,
            "temperature": creativity,
            "messages": messages,
            "stream": true
        });

        println!("Debug: Model parameters - top_p: {}, top_k: {}, frequency_penalty: {}, creativity: {}", top_p, top_k, repetition_penalty, creativity);
        let (tx, rx) = mpsc::channel(1024);
        
        tokio::task::spawn(async move {
            let url = "https://api.fireworks.ai/inference/v1/chat/completions";
            let error_prefix = "Error sending request to Fireworks API";

            let response = client
                .post(url)
                .header("Authorization", format!("Bearer {}", api_key))
                .header("Content-Type", "application/json")
                .json(&json_body)
                .send()
                .await;

            match response {
                Ok(response) => {
                    if !response.status().is_success() {
                        let error_body = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
                        let _ = tx.send(format!("{}: {}", error_prefix, error_body)).await;
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
                                            if let Some(content) = json["choices"][0]["delta"]["content"].as_str() {
                                                if tx.send(content.to_string()).await.is_err() {
                                                    return;
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
                }
                Err(e) => {
                    let _ = tx.send(format!("{}: {}", error_prefix, e)).await;
                }
            }
        });

        Ok(rx)
    }

    fn set_current_model(&self, model: String) {
        let full_model_name = if !model.starts_with("accounts/fireworks/models/") {
            format!("accounts/fireworks/models/{}", model)
        } else {
            model
        };
        *self.current_model.lock().unwrap() = full_model_name.clone();
    }

    fn update_profile(&self, profile: ProfileType) {
        self.base.lock().unwrap().update_profile(profile);
    }

    fn get_parameters(&self) -> (f32, u32, f32, f32) {
        self.base.lock().unwrap().get_parameters()
    }
}

impl fmt::Display for Fireworks {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Fireworks")
    }
}