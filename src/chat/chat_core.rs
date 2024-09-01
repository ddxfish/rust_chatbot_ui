use crate::message::Message;
use crate::chatbot::Chatbot;
use crate::providers::Provider;
use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;
use tokio::sync::mpsc;
use serde_json::json;
use super::history_manager::ChatHistory;
use super::file_operations;

pub struct Chat {
    pub messages: Arc<Mutex<Vec<Message>>>,
    pub chatbot: Arc<Chatbot>,
    pub provider: Arc<dyn Provider + Send + Sync>,
    pub runtime: Runtime,
    pub is_processing: Arc<Mutex<bool>>,
    pub ui_sender: mpsc::UnboundedSender<String>,
    pub ui_receiver: Arc<Mutex<mpsc::UnboundedReceiver<String>>>,
    pub history_manager: Arc<Mutex<ChatHistory>>,
    pub needs_naming: Arc<Mutex<bool>>,
    pub name_sender: mpsc::UnboundedSender<String>,
    pub name_receiver: Arc<Mutex<mpsc::UnboundedReceiver<String>>>,
    pub current_model: Arc<Mutex<String>>,
    pub error_sender: mpsc::UnboundedSender<String>,
    pub error_receiver: Arc<Mutex<mpsc::UnboundedReceiver<String>>>,
    pub has_updates: Arc<Mutex<bool>>,
}

impl Chat {
    pub fn new(initial_provider: Arc<dyn Provider + Send + Sync>) -> Self {
        let (ui_sender, ui_receiver) = mpsc::unbounded_channel();
        let (name_sender, name_receiver) = mpsc::unbounded_channel();
        let (error_sender, error_receiver) = mpsc::unbounded_channel();
        let initial_model = initial_provider.models()[0].to_string();
        Self {
            messages: Arc::new(Mutex::new(Vec::new())),
            chatbot: Arc::new(Chatbot::new(Arc::clone(&initial_provider))),
            runtime: Runtime::new().unwrap(),
            is_processing: Arc::new(Mutex::new(false)),
            ui_sender,
            ui_receiver: Arc::new(Mutex::new(ui_receiver)),
            history_manager: Arc::new(Mutex::new(ChatHistory::new("chat_history"))),
            needs_naming: Arc::new(Mutex::new(true)),
            name_sender,
            name_receiver: Arc::new(Mutex::new(name_receiver)),
            current_model: Arc::new(Mutex::new(initial_model)),
            provider: initial_provider,
            error_sender,
            error_receiver: Arc::new(Mutex::new(error_receiver)),
            has_updates: Arc::new(Mutex::new(true)),
        }
    }

    pub fn update_provider(&mut self, new_provider: Arc<dyn Provider + Send + Sync>) {
        self.provider = new_provider;
        self.set_has_updates();
    }

    pub fn add_message(&self, content: String, is_user: bool) {
        let model = if is_user { None } else { Some(self.get_current_model()) };
        let message = Message::new(content.clone(), is_user, model.clone());
        self.messages.lock().unwrap().push(message);
        if let Err(e) = self.history_manager.lock().unwrap().append_message(&content, is_user, model.as_deref()) {
            eprintln!("Failed to append message to history: {}", e);
        }

        if !is_user && *self.needs_naming.lock().unwrap() {
            self.generate_chat_name();
        }
        self.set_has_updates();
    }

    pub fn get_messages(&self) -> Vec<Message> {
        self.messages.lock().unwrap().clone()
    }

    pub fn is_processing(&self) -> bool {
        *self.is_processing.lock().unwrap()
    }

    pub fn process_input(&self, input: String, model: &str) {
        let input_with_newlines = input.replace("\\n", "\n");
        self.add_message(input_with_newlines.clone(), true);
        *self.is_processing.lock().unwrap() = true;
        let chatbot = Arc::clone(&self.chatbot);
        let is_processing = Arc::clone(&self.is_processing);
        let ui_sender = self.ui_sender.clone();
        let needs_naming = Arc::clone(&self.needs_naming);
        let current_model = Arc::clone(&self.current_model);
        let error_sender = self.error_sender.clone();

        let mut messages_clone = self.messages.lock().unwrap().clone();

        if model.starts_with("accounts/fireworks/models/") {
            messages_clone.insert(0, Message::new(format!("Model: {}", model), false, Some("system".to_string())));
        }

        self.runtime.spawn(async move {
            match chatbot.stream_response(&messages_clone).await {
                Ok(mut rx) => {
                    let mut full_response = String::new();
                    while let Some(chunk) = rx.recv().await {
                        full_response.push_str(&chunk);
                        if ui_sender.send(chunk).is_err() {
                            break;
                        }
                    }

                    if messages_clone.len() == 1 {
                        *needs_naming.lock().unwrap() = true;
                    }

                    let model = chatbot.get_current_model();
                    *current_model.lock().unwrap() = model;
                }
                Err(e) => {
                    let error_message = format!("Error: {}", e);
                    error_sender.send(error_message).unwrap();
                }
            }
            *is_processing.lock().unwrap() = false;
        });
        self.set_has_updates();
    }

    fn generate_chat_name(&self) {
        let chatbot = Arc::clone(&self.chatbot);
        let messages = Arc::clone(&self.messages);
        let name_sender = self.name_sender.clone();
        let needs_naming = Arc::clone(&self.needs_naming);

        self.runtime.spawn(async move {
            let current_messages = messages.lock().unwrap().clone();
            match chatbot.generate_chat_name(&current_messages).await {
                Ok(name) => {
                    if name_sender.send(name).is_err() {
                        eprintln!("Error: Failed to send generated chat name");
                    }
                }
                Err(e) => eprintln!("Error: Failed to generate chat name: {}", e),
            }
            *needs_naming.lock().unwrap() = false;
        });
    }


}