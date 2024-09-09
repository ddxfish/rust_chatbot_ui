use crate::message::Message;
use crate::chatbot::Chatbot;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;

pub fn generate_chat_name(
    chatbot: Arc<Chatbot>,
    messages: Arc<Mutex<Vec<Message>>>,
    name_sender: mpsc::UnboundedSender<String>,
    runtime: &tokio::runtime::Runtime,
) {
    runtime.spawn(async move {
        let current_messages = messages.lock().unwrap().clone();
        match chatbot.generate_chat_name(&current_messages).await {
            Ok(name) => {
                if name_sender.send(name).is_err() {
                    eprintln!("Error: Failed to send generated chat name");
                }
            }
            Err(e) => eprintln!("Error: Failed to generate chat name: {}", e),
        }
    });
}