use crate::message::Message;
use std::sync::{Arc, Mutex};
use super::history::History;

pub struct MessageHandler;

impl MessageHandler {
    pub fn new() -> Self {
        Self
    }

    pub fn add_message(&self, messages: &Arc<Mutex<Vec<Message>>>, history: &History, content: String, is_user: bool) {
        let message = Message::new(content.clone(), is_user);
        messages.lock().unwrap().push(message);
        if let Err(e) = history.append_message(&content, is_user) {
            eprintln!("Failed to append message to history: {}", e);
        }
    }

    pub fn get_messages(&self, messages: &Arc<Mutex<Vec<Message>>>) -> Vec<Message> {
        messages.lock().unwrap().clone()
    }
}