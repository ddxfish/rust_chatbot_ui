pub mod fireworks;

use std::fmt::Display;
use serde_json::Value;
use std::error::Error;

#[async_trait::async_trait]
pub trait Provider: Display {
    fn name(&self) -> &'static str;
    fn models(&self) -> Vec<&'static str>;
    async fn generate_response(&self, messages: Vec<Value>) -> Result<String, Box<dyn Error>>;
}

pub fn get_providers(api_key: String) -> Vec<Box<dyn Provider + Send + Sync>> {
    vec![
        Box::new(fireworks::Fireworks::new(api_key)),
        // Add other providers here
    ]
}