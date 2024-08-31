pub mod fireworks;
pub mod claude;
pub mod none;
pub mod gpt;

use std::fmt::Display;
use serde_json::Value;
use tokio::sync::mpsc;

#[derive(Debug)]
pub enum ProviderError {
    RequestError(String),
    ResponseError(String),
    StreamError(String),
}

impl std::error::Error for ProviderError {}

impl Display for ProviderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProviderError::RequestError(e) => write!(f, "Request error: {}", e),
            ProviderError::ResponseError(e) => write!(f, "Response error: {}", e),
            ProviderError::StreamError(e) => write!(f, "Stream error: {}", e),
        }
    }
}

#[async_trait::async_trait]
pub trait Provider: Display {
    fn name(&self) -> &'static str;
    fn models(&self) -> Vec<&'static str>;
    async fn stream_response(&self, messages: Vec<Value>) -> Result<mpsc::Receiver<String>, ProviderError>;
}

pub fn get_providers(api_keys: String) -> Vec<Box<dyn Provider + Send + Sync>> {
    let keys: Vec<String> = api_keys.split(',').map(String::from).collect();
    vec![
        Box::new(none::None::new()),
        Box::new(fireworks::Fireworks::new(keys.get(0).cloned().unwrap_or_default())),
        Box::new(claude::Claude::new(keys.get(1).cloned().unwrap_or_default())),
        Box::new(gpt::GPT::new(keys.get(2).cloned().unwrap_or_default())),
    ]
}