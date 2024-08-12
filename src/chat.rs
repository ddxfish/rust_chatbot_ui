use crate::message::Message;
use crate::chatbot::Chatbot;
use crate::chat_history::ChatHistory;
use crate::providers::Provider;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use tokio::runtime::Runtime;
use std::sync::Arc;

const MESSAGE_SEPARATOR: &str = "\n<<<MESSAGE_SEPARATOR>>>\n";

pub struct Chat {
    messages: Vec<Message>,
    chatbot: Chatbot,
    chat_history: ChatHistory,
    runtime: Runtime,
}

impl Chat {
    pub fn new(provider: Arc<dyn Provider + Send + Sync>) -> Self {
        Self {
            messages: Vec::new(),
            chatbot: Chatbot::new(provider),
            chat_history: ChatHistory::new("chat_history"),
            runtime: Runtime::new().unwrap(),
        }
    }

    pub fn add_message(&mut self, content: String, is_user: bool) {
        let message = Message::new(content.clone(), is_user);
        self.messages.push(message);
        if let Err(e) = self.chat_history.append_message(&content, is_user) {
            eprintln!("Failed to append message to history: {}", e);
        }
    }

    pub fn get_messages(&self) -> &Vec<Message> {
        &self.messages
    }

    pub fn process_input(&mut self, input: String) {
        self.add_message(input.clone(), true);
        let response = self.runtime.block_on(self.chatbot.generate_response(&self.messages));
        self.add_message(response, false);
    }

    pub fn create_new_chat(&mut self) -> Result<(), std::io::Error> {
        self.messages.clear();
        self.chat_history.create_new_chat()
    }

    pub fn load_chat(&mut self, file_name: &str) -> Result<(), std::io::Error> {
        let content = self.chat_history.load_chat(file_name)?;
        self.messages.clear();
        for message in content.split(MESSAGE_SEPARATOR) {
            let trimmed = message.trim();
            if !trimmed.is_empty() {
                if let Some(content) = trimmed.strip_prefix("User: ") {
                    self.messages.push(Message::new(content.to_string(), true));
                } else if let Some(content) = trimmed.strip_prefix("Bot: ") {
                    self.messages.push(Message::new(content.to_string(), false));
                }
            }
        }
        Ok(())
    }

    pub fn get_history_files(&self) -> &Vec<String> {
        self.chat_history.get_history_files()
    }

    pub fn get_current_file(&self) -> Option<&String> {
        self.chat_history.get_current_file()
    }

    pub fn delete_chat(&mut self, file_name: &str) -> Result<(), std::io::Error> {
        self.chat_history.delete_chat(file_name)
    }

    pub fn export_chat(&self, path: &Path) -> Result<(), std::io::Error> {
        let mut file = File::create(path)?;
        for message in &self.messages {
            let prefix = if message.is_user() { "User: " } else { "Bot: " };
            writeln!(file, "{}{}{}", prefix, message.content(), MESSAGE_SEPARATOR)?;
        }
        Ok(())
    }
}