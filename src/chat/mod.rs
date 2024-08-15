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
    needs_naming: Arc<Mutex<bool>>,
    name_sender: mpsc::UnboundedSender<String>,
    name_receiver: Arc<Mutex<mpsc::UnboundedReceiver<String>>>,
}

impl Chat {
    pub fn new(provider: Arc<dyn Provider + Send + Sync>) -> Self {
        let (ui_sender, ui_receiver) = mpsc::unbounded_channel();
        let (name_sender, name_receiver) = mpsc::unbounded_channel();
        Self {
            messages: Arc::new(Mutex::new(Vec::new())),
            chatbot: Arc::new(Chatbot::new(provider)),
            runtime: Runtime::new().unwrap(),
            is_processing: Arc::new(Mutex::new(false)),
            ui_sender,
            ui_receiver: Arc::new(Mutex::new(ui_receiver)),
            history: Arc::new(Mutex::new(history::ChatHistory::new("chat_history"))),
            needs_naming: Arc::new(Mutex::new(true)),
            name_sender,
            name_receiver: Arc::new(Mutex::new(name_receiver)),
        }
    }

    pub fn add_message(&self, content: String, is_user: bool) {
        println!("Debug: Adding message to memory: {} (User: {})", content, is_user);
        let message = Message::new(content.clone(), is_user);
        self.messages.lock().unwrap().push(message);
        println!("Debug: Writing message to history file");
        if let Err(e) = self.history.lock().unwrap().append_message(&content, is_user) {
            eprintln!("Failed to append message to history: {}", e);
        }

        if !is_user && *self.needs_naming.lock().unwrap() {
            self.generate_chat_name();
        }
    }

    pub fn get_messages(&self) -> Vec<Message> {
        self.messages.lock().unwrap().clone()
    }

    pub fn is_processing(&self) -> bool {
        *self.is_processing.lock().unwrap()
    }

    pub fn process_input(&self, input: String) {
        println!("Debug: Processing input: {}", input);
        self.add_message(input.clone(), true);
        *self.is_processing.lock().unwrap() = true;
        let chatbot = Arc::clone(&self.chatbot);
        let is_processing = Arc::clone(&self.is_processing);
        let ui_sender = self.ui_sender.clone();
        let needs_naming = Arc::clone(&self.needs_naming);

        let messages_clone = self.messages.lock().unwrap().clone();

        self.runtime.spawn(async move {
            println!("Debug: Starting to stream response");
            if let Ok(mut rx) = chatbot.stream_response(&messages_clone).await {
                println!("Debug: Successfully got stream response");
                let mut full_response = String::new();
                while let Some(chunk) = rx.recv().await {
                    println!("Debug: Received chunk: {}", chunk);
                    full_response.push_str(&chunk);
                    if ui_sender.send(chunk).is_err() {
                        println!("Debug: Error sending chunk to UI");
                        break;
                    }
                }
                println!("Debug: Finished streaming response");
                
                // Set needs_naming to true after the first bot response
                if messages_clone.len() == 1 {
                    *needs_naming.lock().unwrap() = true;
                    println!("Debug: Set needs_naming to true");
                }
            } else {
                println!("Debug: Failed to get stream response");
            }
            *is_processing.lock().unwrap() = false;
        });
    }

    fn generate_chat_name(&self) {
        let chatbot = Arc::clone(&self.chatbot);
        let messages = Arc::clone(&self.messages);
        let name_sender = self.name_sender.clone();
        let needs_naming = Arc::clone(&self.needs_naming);

        self.runtime.spawn(async move {
            println!("Debug: Generating chat name");
            let current_messages = messages.lock().unwrap().clone();
            match chatbot.generate_chat_name(&current_messages).await {
                Ok(name) => {
                    println!("Debug: Generated chat name: {}", name);
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

    pub fn create_new_chat(&self) -> Result<(), std::io::Error> {
        println!("Debug: Creating new chat");
        self.messages.lock().unwrap().clear();
        *self.needs_naming.lock().unwrap() = true;
        self.history.lock().unwrap().create_new_chat()
    }

    pub fn load_chat(&self, file_name: &str) -> Result<(), std::io::Error> {
        println!("Debug: Loading chat: {}", file_name);
        *self.needs_naming.lock().unwrap() = false;
        self.history.lock().unwrap().load_chat(file_name, &mut self.messages.lock().unwrap())
    }

    pub fn get_history_files(&self) -> Vec<String> {
        self.history.lock().unwrap().get_history_files()
    }

    pub fn get_current_file(&self) -> Option<String> {
        self.history.lock().unwrap().get_current_file()
    }

    pub fn delete_chat(&self, file_name: &str) -> Result<(), std::io::Error> {
        println!("Debug: Deleting chat: {}", file_name);
        self.history.lock().unwrap().delete_chat(file_name)
    }

    pub fn export_chat(&self, path: &std::path::Path) -> Result<(), std::io::Error> {
        println!("Debug: Exporting chat to: {:?}", path);
        self.history.lock().unwrap().export_chat(path, &self.messages.lock().unwrap())
    }

    pub fn rename_current_chat(&self, new_name: &str) -> Result<(), std::io::Error> {
        self.history.lock().unwrap().rename_current_chat(new_name)
    }
}