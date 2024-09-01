use crate::message::Message;
use crate::providers::{Provider, ProviderError};
use serde_json::json;
use std::sync::Arc;
use tokio::sync::mpsc;

pub struct Chatbot {
    provider: Arc<dyn Provider + Send + Sync>,
    current_model: String,
}

impl Chatbot {
    pub fn new(provider: Arc<dyn Provider + Send + Sync>) -> Self {
        let initial_model = provider.models()[0].to_string();
        Self { 
            provider,
            current_model: initial_model,
        }
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
            "No intro text or confirmation, just give me a concise 3-word name for this chat. Your response should be 3 words max. If you don't have enough info, be a bit creative. Use initcaps:\n\n{}",
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

        let processed_name = name.trim();
        let final_name = limit_to_three_words(&processed_name);

        println!("Debug: Generated chat name: {}", final_name);

        Ok(final_name)
    }

    pub fn get_current_model(&self) -> String {
        self.current_model.clone()
    }

    pub fn switch_model(&mut self, providers: &Vec<Arc<dyn Provider + Send + Sync>>, model: String) {
        if let Some(new_provider) = providers.iter().find(|p| p.models().contains(&model.as_str())) {
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
                panic!("Fireworks provider not found for custom model: {}", model);
            }
        } else {
            panic!("Model not found: {}", model);
        }
    }
}

fn limit_to_three_words(s: &str) -> String {
    let mut words = s.split_whitespace();
    let word1 = words.next().unwrap_or("");
    let word2 = words.next().unwrap_or("");
    let word3 = words.next().unwrap_or("");

    if word3.is_empty() {
        format!("{} {}", word1, word2).trim().to_string()
    } else {
        format!("{} {} {}", word1, word2, word3)
    }
}