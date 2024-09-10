use super::{ProviderError, ProviderTrait, BaseProvider};
use serde_json::{json, Value};
use tokio::sync::mpsc;
use std::fmt;
use std::sync::Arc;

pub struct Fireworks {
    base: Arc<BaseProvider>,
}

impl Fireworks {
    pub fn new(api_key: String) -> Self {
        Self {
            base: Arc::new(BaseProvider::new(api_key)),
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
            "Other",
        ]
    }

    fn stream_response(&self, messages: Vec<Value>) -> Result<mpsc::Receiver<String>, ProviderError> {
        let (model, api_messages) = if messages.first().and_then(|m| m["role"].as_str()) == Some("system") &&
                                       messages.first().and_then(|m| m["content"].as_str()).map_or(false, |c| c.starts_with("Model: ")) {
            let custom_model = messages[0]["content"].as_str().unwrap().strip_prefix("Model: ").unwrap();
            (format!("accounts/fireworks/models/{}", custom_model), messages[1..].to_vec())
        } else {
            ("accounts/fireworks/models/llama-v3p1-70b-instruct".to_string(), messages)
        };

        let json_body = json!({
            "model": model,
            "max_tokens": 16384,
            "top_p": 1,
            "top_k": 40,
            "presence_penalty": 0,
            "frequency_penalty": 0,
            "temperature": 0.6,
            "messages": api_messages,
            "stream": true
        });

        let (tx, rx) = mpsc::channel(1024);
        let base = self.base.clone();
        
        tokio::spawn(async move {
            match base.stream_response(
                "https://api.fireworks.ai/inference/v1/chat/completions",
                json_body,
                "Error sending request to Fireworks API"
            ).await {
                Ok(mut stream) => {
                    while let Some(chunk) = stream.recv().await {
                        if tx.send(chunk).await.is_err() {
                            break;
                        }
                    }
                }
                Err(e) => {
                    let _ = tx.send(format!("Error: {}", e)).await;
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