use crate::message::Message;
use crate::chatbot::Chatbot;
use crate::chat_history::ChatHistory;
use crate::providers::Provider;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use tokio::runtime::Runtime;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;

const MESSAGE_SEPARATOR: &str = "\n<<<MESSAGE_SEPARATOR>>>\n";

pub struct Chat {
    messages: Arc<Mutex<Vec<Message>>>,
    chatbot: Arc<Chatbot>,
    chat_history: Arc<Mutex<ChatHistory>>,
    runtime: Runtime,
    is_processing: Arc<Mutex<bool>>,
    stream: Arc<Mutex<Option<mpsc::Receiver<String>>>>,
}

impl Chat {
    pub fn new(provider: Arc<dyn Provider + Send + Sync>) -> Self {
        Self {
            messages: Arc::new(Mutex::new(Vec::new())),
            chatbot: Arc::new(Chatbot::new(provider)),
            chat_history: Arc::new(Mutex::new(ChatHistory::new("chat_history"))),
            runtime: Runtime::new().unwrap(),
            is_processing: Arc::new(Mutex::new(false)),
            stream: Arc::new(Mutex::new(None)),
        }
    }

    pub fn add_message(&self, content: String, is_user: bool) {
        let message = Message::new(content.clone(), is_user);
        self.messages.lock().unwrap().push(message);
        if let Err(e) = self.chat_history.lock().unwrap().append_message(&content, is_user) {
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
        println!("Processing input: {}", input);
        self.add_message(input.clone(), true);
        *self.is_processing.lock().unwrap() = true;
        let chatbot = Arc::clone(&self.chatbot);
        let messages = Arc::clone(&self.messages);
        let stream = Arc::clone(&self.stream);
        let is_processing = Arc::clone(&self.is_processing);

        self.runtime.spawn(async move {
            println!("Starting to generate response");
            let messages = messages.lock().unwrap().clone();
            match chatbot.stream_response(&messages).await {
                Ok(rx) => {
                    println!("Response stream received");
                    *stream.lock().unwrap() = Some(rx);
                },
                Err(e) => {
                    eprintln!("Error generating response: {:?}", e);
                }
            }
            *is_processing.lock().unwrap() = false;
            println!("Finished processing");
        });
    }

    pub fn check_stream(&self) -> Option<String> {
        if let Some(rx) = &mut *self.stream.lock().unwrap() {
            match rx.try_recv() {
                Ok(chunk) => {
                    println!("Received chunk: {}", chunk);
                    Some(chunk)
                },
                Err(e) => {
                    match e {
                        tokio::sync::mpsc::error::TryRecvError::Empty => None,
                        tokio::sync::mpsc::error::TryRecvError::Disconnected => {
                            println!("Stream disconnected");
                            *self.stream.lock().unwrap() = None;
                            *self.is_processing.lock().unwrap() = false;
                            None
                        }
                    }
                }
            }
        } else {
            None
        }
    }

    pub fn create_new_chat(&self) -> Result<(), std::io::Error> {
        self.messages.lock().unwrap().clear();
        self.chat_history.lock().unwrap().create_new_chat()
    }

    pub fn load_chat(&self, file_name: &str) -> Result<(), std::io::Error> {
        let content = self.chat_history.lock().unwrap().load_chat(file_name)?;
        let mut messages = self.messages.lock().unwrap();
        messages.clear();
        for message in content.split(MESSAGE_SEPARATOR) {
            let trimmed = message.trim();
            if !trimmed.is_empty() {
                if let Some(content) = trimmed.strip_prefix("User: ") {
                    messages.push(Message::new(content.to_string(), true));
                } else if let Some(content) = trimmed.strip_prefix("Bot: ") {
                    messages.push(Message::new(content.to_string(), false));
                }
            }
        }
        Ok(())
    }

    pub fn get_history_files(&self) -> Vec<String> {
        self.chat_history.lock().unwrap().get_history_files().clone()
    }

    pub fn get_current_file(&self) -> Option<String> {
        self.chat_history.lock().unwrap().get_current_file().cloned()
    }

    pub fn delete_chat(&self, file_name: &str) -> Result<(), std::io::Error> {
        self.chat_history.lock().unwrap().delete_chat(file_name)
    }

    pub fn export_chat(&self, path: &Path) -> Result<(), std::io::Error> {
        let mut file = File::create(path)?;
        for message in self.messages.lock().unwrap().iter() {
            let prefix = if message.is_user() { "User: " } else { "Bot: " };
            writeln!(file, "{}{}{}", prefix, message.content(), MESSAGE_SEPARATOR)?;
        }
        Ok(())
    }
}