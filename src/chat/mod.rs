pub mod history_manager;
pub mod file_operations;
pub mod ui_rendering;

use crate::message::Message;
use crate::chatbot::Chatbot;
use crate::providers::Provider;
use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;
use tokio::sync::mpsc;
use serde_json::json;

pub struct Chat {
    messages: Arc<Mutex<Vec<Message>>>,
    pub chatbot: Arc<Chatbot>,
    pub provider: Arc<dyn Provider + Send + Sync>,
    runtime: Runtime,
    is_processing: Arc<Mutex<bool>>,
    ui_sender: mpsc::UnboundedSender<String>,
    ui_receiver: Arc<Mutex<mpsc::UnboundedReceiver<String>>>,
    history_manager: Arc<Mutex<history_manager::ChatHistory>>,
    needs_naming: Arc<Mutex<bool>>,
    name_sender: mpsc::UnboundedSender<String>,
    name_receiver: Arc<Mutex<mpsc::UnboundedReceiver<String>>>,
    pub current_model: Arc<Mutex<String>>,
    error_sender: mpsc::UnboundedSender<String>,
    error_receiver: Arc<Mutex<mpsc::UnboundedReceiver<String>>>,
    has_updates: Arc<Mutex<bool>>,
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
            history_manager: Arc::new(Mutex::new(history_manager::ChatHistory::new("chat_history"))),
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

    pub fn check_ui_updates(&self) -> Option<String> {
        self.ui_receiver.lock().unwrap().try_recv().ok()
    }

    pub fn check_name_updates(&self) -> Option<String> {
        self.name_receiver.lock().unwrap().try_recv().ok()
    }

    pub fn check_error_updates(&self) -> Option<String> {
        self.error_receiver.lock().unwrap().try_recv().ok()
    }

    pub fn create_new_chat(&self) -> Result<(), std::io::Error> {
        self.messages.lock().unwrap().clear();
        *self.needs_naming.lock().unwrap() = true;
        self.history_manager.lock().unwrap().create_new_chat()?;
        self.set_has_updates();
        Ok(())
    }

    pub fn load_chat(&self, file_name: &str) -> Result<(), std::io::Error> {
        *self.needs_naming.lock().unwrap() = false;
        self.history_manager.lock().unwrap().load_chat(file_name, &mut self.messages.lock().unwrap())?;
        self.set_has_updates();
        Ok(())
    }

    pub fn get_history_files(&self) -> Vec<String> {
        self.history_manager.lock().unwrap().get_history_files()
    }

    pub fn get_current_file(&self) -> Option<String> {
        self.history_manager.lock().unwrap().get_current_file()
    }

    pub fn delete_chat(&self, file_name: &str) -> Result<(), std::io::Error> {
        self.history_manager.lock().unwrap().delete_chat(file_name)?;
        self.set_has_updates();
        Ok(())
    }

    pub fn export_chat(&self, path: &std::path::Path) -> Result<(), std::io::Error> {
        file_operations::export_chat(path, &self.messages.lock().unwrap())
    }
    

    pub fn rename_current_chat(&self, new_name: &str) -> Result<(), std::io::Error> {
        self.history_manager.lock().unwrap().rename_current_chat(new_name)?;
        self.set_has_updates();
        Ok(())
    }

    pub fn get_current_model(&self) -> String {
        self.current_model.lock().unwrap().clone()
    }

    pub fn set_current_model(&self, model: &str) {
        *self.current_model.lock().unwrap() = model.to_string();
        self.set_has_updates();
    }

    pub fn load_most_recent_or_create_new(&self) -> Result<(), std::io::Error> {
        let history = self.history_manager.lock().unwrap();
        let files = history.get_history_files();
        drop(history);  // Release the lock before calling other methods

        if let Some(most_recent) = files.first() {
            self.load_chat(most_recent)
        } else {
            self.create_new_chat()
        }
    }

    pub fn has_updates(&self) -> bool {
        let mut has_updates = self.has_updates.lock().unwrap();
        let updates = *has_updates;
        *has_updates = false;
        updates
    }

    fn set_has_updates(&self) {
        *self.has_updates.lock().unwrap() = true;
    }
}