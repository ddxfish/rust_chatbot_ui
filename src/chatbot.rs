use crate::message::Message;
use crate::providers::{ProviderTrait, ProviderError};
use serde_json::json;
use std::sync::Arc;
use tokio::sync::mpsc;

pub struct Chatbot {
    provider: Arc<dyn ProviderTrait + Send + Sync>,
    current_model: String,
}

impl Chatbot {
    pub fn new(provider: Arc<dyn ProviderTrait + Send + Sync>) -> Self {
        let initial_model = provider.models()[0].to_string();
        Self { 
            provider,
            current_model: initial_model,
        }
    }

    pub fn stream_response(&self, messages: &Vec<Message>) -> Result<mpsc::Receiver<String>, ProviderError> {
        println!("Debug: Streaming response for {} messages", messages.len());
        let formatted_messages = messages.iter().map(|m| {
            json!({
                "role": if m.is_user() { "user" } else { "assistant" },
                "content": m.content()
            })
        }).collect::<Vec<_>>();

        self.provider.stream_response(formatted_messages)
    }

    pub fn generate_chat_name(&self, messages: &Vec<Message>) -> Result<mpsc::Receiver<String>, ProviderError> {
        println!("Debug: Generating chat name for {} messages", messages.len());
        let prompt = format!(
            "No intro text or confirmation, just give me a concise 3-word name for this chat. Your response should be 3 words max. If you don't have enough info, be a bit creative. Use initcaps:\n\n{}",
            messages.iter().map(|m| format!("{}: {}", if m.is_user() { "User" } else { "Assistant" }, m.content())).collect::<Vec<_>>().join("\n")
        );

        let formatted_message = vec![json!({
            "role": "user",
            "content": prompt
        })];

        self.provider.stream_response(formatted_message)
    }

    pub fn switch_model(&mut self, providers: &Vec<Arc<dyn ProviderTrait + Send + Sync>>, model: String) {
        if model == "Other" {
            // For "Other", we keep the current provider but update the model name
            self.current_model = model;
            println!("Debug: Switched to custom model: {}", self.current_model);
        } else if let Some(new_provider) = providers.iter().find(|p| p.models().contains(&model.as_str())) {
            self.provider = Arc::clone(new_provider);
            self.current_model = model;
            println!("Debug: Switched to model: {}", self.current_model);
        } else if model.starts_with("accounts/fireworks/models/") {
            // Handle custom Fireworks model
            if let Some(fireworks_provider) = providers.iter().find(|p| p.name() == "Fireworks") {
                self.provider = Arc::clone(fireworks_provider);
                self.current_model = model;
                println!("Debug: Switched to custom Fireworks model: {}", self.current_model);
            } else {
                eprintln!("Fireworks provider not found for custom model: {}", model);
            }
        } else {
            eprintln!("Model not found: {}", model);
        }
    }
}