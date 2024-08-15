use crate::message::Message;
use crate::providers::{Provider, ProviderError};
use serde_json::json;
use std::sync::Arc;
use tokio::sync::mpsc;

pub struct Chatbot {
    provider: Arc<dyn Provider + Send + Sync>,
}

impl Chatbot {
    pub fn new(provider: Arc<dyn Provider + Send + Sync>) -> Self {
        Self { provider }
    }

    pub async fn stream_response(&self, messages: &Vec<Message>) -> Result<mpsc::Receiver<String>, ProviderError> {
        println!("Debug: Streaming response for {} messages", messages.len());
        let formatted_messages = messages.iter().map(|m| {
            json!({
                "role": if m.is_user() { "user" } else { "assistant" },
                "content": m.content()
            })
        }).collect::<Vec<_>>();

        self.provider.stream_response(formatted_messages).await
    }

    pub async fn generate_chat_name(&self, messages: &Vec<Message>) -> Result<String, ProviderError> {
        println!("Debug: Generating chat name for {} messages", messages.len());
        let prompt = format!(
            "Based on the following conversation, generate a concise 2-3 word name for this chat:\n\n{}",
            messages.iter().map(|m| format!("{}: {}", if m.is_user() { "User" } else { "Assistant" }, m.content())).collect::<Vec<_>>().join("\n")
        );

        let formatted_message = vec![json!({
            "role": "user",
            "content": prompt
        })];

        let mut rx = self.provider.stream_response(formatted_message).await?;
        let mut name = String::new();

        println!("Debug: Waiting for chat name response");
        while let Some(chunk) = rx.recv().await {
            println!("Debug: Received name chunk: {}", chunk);
            name.push_str(&chunk);
        }

        println!("Debug: Generated chat name: {}", name.trim());
        Ok(name.trim().to_string())
    }
}