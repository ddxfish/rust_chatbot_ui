use super::{Provider, ProviderError};
use std::fmt;
use serde_json::Value;
use tokio::sync::mpsc;

pub struct None;

impl None {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl Provider for None {
    fn name(&self) -> &'static str {
        "None"
    }

    fn models(&self) -> Vec<&'static str> {
        vec!["Hello"]
    }

    async fn stream_response(&self, _messages: Vec<Value>) -> Result<mpsc::Receiver<String>, ProviderError> {
        let (tx, rx) = mpsc::channel(1);
        tokio::spawn(async move {
            let _ = tx.send("Hello".to_string()).await;
        });
        Ok(rx)
    }
}

impl fmt::Display for None {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "None")
    }
}