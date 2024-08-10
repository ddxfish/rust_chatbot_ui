use crate::message::Message;
use crate::providers::Provider;
use serde_json::json;
use std::sync::Arc;

pub struct Chatbot {
    provider: Arc<dyn Provider + Send + Sync>,
}

impl Chatbot {
    pub fn new(provider: Arc<dyn Provider + Send + Sync>) -> Self {
        Self { provider }
    }

    pub async fn generate_response(&self, messages: &Vec<Message>) -> String {
        let formatted_messages = messages.iter().map(|m| {
            json!({
                "role": if m.is_user() { "user" } else { "assistant" },
                "content": m.content()
            })
        }).collect::<Vec<_>>();

        match self.provider.generate_response(formatted_messages).await {
            Ok(response) => response,
            Err(e) => format!("Error generating response: {}", e),
        }
    }
}