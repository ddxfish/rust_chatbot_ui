pub mod fireworks;
pub mod claude;
pub mod none;
pub mod gpt;
pub mod base_provider;

use std::fmt::Display;


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

pub use base_provider::{BaseProvider, ProviderTrait};

pub fn get_providers(api_keys: String) -> Vec<Box<dyn ProviderTrait + Send + Sync>> {
    let keys: Vec<String> = api_keys.split(',').map(String::from).collect();
    vec![
        Box::new(none::None::new()),
        Box::new(fireworks::Fireworks::new(keys.get(0).cloned().unwrap_or_default())),
        Box::new(claude::Claude::new(keys.get(1).cloned().unwrap_or_default())),
        Box::new(gpt::GPT::new(keys.get(2).cloned().unwrap_or_default())),
    ]
}
