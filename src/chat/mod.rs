mod history;

use crate::message::Message;
use crate::chatbot::Chatbot;
use crate::providers::Provider;
use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;
use tokio::sync::mpsc;

pub struct Chat {
    messages: Arc<Mutex<Vec<Message>>>,
    chatbot: Arc<Chatbot>,
    runtime: Runtime,
    is_processing: Arc<Mutex<bool>>,
    ui_sender: mpsc::UnboundedSender<String>,
    ui_receiver: Arc<Mutex<mpsc::UnboundedReceiver<String>>>,
    history: Arc<Mutex<history::ChatHistory>>,
}

impl Chat {
    pub fn new(provider: Arc<dyn Provider + Send + Sync>) -> Self {
        let (ui_sender, ui_receiver) = mpsc::unbounded_channel();
        Self {
            messages: Arc::new(Mutex::new(Vec::new())),
            chatbot: Arc::new(Chatbot::new(provider)),
            runtime: Runtime::new().unwrap(),
            is_processing: Arc::new(Mutex::new(false)),
            ui_sender,
            ui_receiver: Arc::new(Mutex::new(ui_receiver)),
            history: Arc::new(Mutex::new(history::ChatHistory::new("chat_history"))),
        }
    }

    pub fn add_message(&self, content: String, is_user: bool) {
        let message = Message::new(content.clone(), is_user);
        self.messages.lock().unwrap().push(message);
        if let Err(e) = self.history.lock().unwrap().append_message(&content, is_user) {
            eprintln!("Failed to append message to history: {}", e);
        }
    }

    pub fn get_messages(&self) -> Vec<Message> {
        self.messages.lock().unwrap().clone()
    }

    pub fn is_processing(&self) -> bool {
        *self.is_processing.lock().unwrap()
    }

    pub fn process_input(&self, input: String) {
        self.add_message(input.clone(), true);
        *self.is_processing.lock().unwrap() = true;
        let chatbot = Arc::clone(&self.chatbot);
        let messages = Arc::clone(&self.messages);
        let is_processing = Arc::clone(&self.is_processing);
        let ui_sender = self.ui_sender.clone();

        self.runtime.spawn(async move {
            let messages_clone = messages.lock().unwrap().clone();
            if let Ok(mut rx) = chatbot.stream_response(&messages_clone).await {
                while let Some(chunk) = rx.recv().await {
                    if ui_sender.send(chunk).is_err() {
                        break;
                    }
                }
            }
            *is_processing.lock().unwrap() = false;
        });
    }

    pub fn check_ui_updates(&self) -> Option<String> {
        self.ui_receiver.lock().unwrap().try_recv().ok()
    }

    pub fn create_new_chat(&self) -> Result<(), std::io::Error> {
        self.messages.lock().unwrap().clear();
        self.history.lock().unwrap().create_new_chat()
    }

    pub fn load_chat(&self, file_name: &str) -> Result<(), std::io::Error> {
        self.history.lock().unwrap().load_chat(file_name, &mut self.messages.lock().unwrap())
    }

    pub fn get_history_files(&self) -> Vec<String> {
        self.history.lock().unwrap().get_history_files()
    }

    pub fn get_current_file(&self) -> Option<String> {
        self.history.lock().unwrap().get_current_file()
    }

    pub fn delete_chat(&self, file_name: &str) -> Result<(), std::io::Error> {
        self.history.lock().unwrap().delete_chat(file_name)
    }

    pub fn export_chat(&self, path: &std::path::Path) -> Result<(), std::io::Error> {
        self.history.lock().unwrap().export_chat(path, &self.messages.lock().unwrap())
    }
}