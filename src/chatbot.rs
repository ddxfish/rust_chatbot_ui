use crate::message::Message;
use crate::providers::Provider;
use serde_json::json;
use std::sync::Arc;
use tokio::sync::mpsc;
use std::error::Error;

pub struct Chatbot {
    provider: Arc<dyn Provider + Send + Sync>,
}

impl Chatbot {
    pub fn new(provider: Arc<dyn Provider + Send + Sync>) -> Self {
        Self { provider }
    }

    pub async fn stream_response(&self, messages: &Vec<Message>) -> Result<mpsc::Receiver<String>, Box<dyn Error>> {
        let formatted_messages = messages.iter().map(|m| {
            json!({
                "role": if m.is_user() { "user" } else { "assistant" },
                "content": m.content()
            })
        }).collect::<Vec<_>>();

        self.provider.stream_response(formatted_messages).await
    }
}