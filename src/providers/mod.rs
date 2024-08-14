pub mod fireworks;

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
    //async fn generate_response(&self, messages: Vec<Value>) -> Result<String, ProviderError>;
    async fn stream_response(&self, messages: Vec<Value>) -> Result<mpsc::Receiver<String>, ProviderError>;
}

pub fn get_providers(api_key: String) -> Vec<Box<dyn Provider + Send + Sync>> {
    vec![
        Box::new(fireworks::Fireworks::new(api_key)),
        // Add other providers here
    ]
}