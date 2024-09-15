use super::{ProviderError, ProviderTrait, BaseProvider};
use serde_json::{json, Value};
use tokio::sync::mpsc;
use std::fmt;
use std::sync::{Arc, Mutex};

pub struct Fireworks {
    base: Arc<BaseProvider>,
    current_model: Arc<Mutex<String>>,
}

impl Fireworks {
    pub fn new(api_key: String) -> Self {
        Self {
            base: Arc::new(BaseProvider::new(api_key)),
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
            "Other",
        ]
    }

    fn stream_response(&self, messages: Vec<Value>) -> Result<mpsc::Receiver<String>, ProviderError> {
        let model = self.current_model.lock().unwrap().clone();
        let json_body = json!({
            "model": model,
            "max_tokens": 16384,
            "top_p": 0.9,
            "top_k": 40,
            "presence_penalty": 0,
            "frequency_penalty": 0,
            "temperature": 0.6,
            "messages": messages,
            "stream": true
        });
        println!("Debug: Fireworks model response: {:?}", json_body["model"]);

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

    fn set_current_model(&self, model: String) {
        let full_model_name = if !model.starts_with("accounts/fireworks/models/") {
            format!("accounts/fireworks/models/{}", model)
        } else {
            model
        };
        *self.current_model.lock().unwrap() = full_model_name.clone();
        println!("Debug: Fireworks model set to {}", full_model_name);
    }
}

impl fmt::Display for Fireworks {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Fireworks")
    }
}