use super::{ProviderError, ProviderTrait, BaseProvider};
use serde_json::{json, Value};
use tokio::sync::mpsc;
use std::fmt;
use std::sync::Arc;

pub struct GPT {
    base: Arc<BaseProvider>,
}

impl GPT {
    pub fn new(api_key: String) -> Self {
        Self {
            base: Arc::new(BaseProvider::new(api_key)),
        }
    }
}

impl ProviderTrait for GPT {
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

    fn stream_response(&self, messages: Vec<Value>) -> Result<mpsc::Receiver<String>, ProviderError> {
        let json_body = json!({
            "model": "gpt-3.5-turbo",
            "messages": messages,
            "stream": true
        });

        let (tx, rx) = mpsc::channel(1024);
        let base = self.base.clone();
        
        tokio::spawn(async move {
            match base.stream_response(
                "https://api.openai.com/v1/chat/completions",
                json_body,
                "Error sending request to GPT API"
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

impl fmt::Display for GPT {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "GPT")
    }
}