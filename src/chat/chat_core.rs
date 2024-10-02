use crate::message::Message;
use crate::chatbot::Chatbot;
use crate::providers::ProviderTrait;
use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;
use tokio::sync::mpsc;
use std::sync::atomic::{AtomicBool, Ordering};
use super::history_manager::ChatHistory;
use super::chat_name_generation;
use crate::app::ProfileType;
use crate::ui::MessageView;

pub struct Chat {
    pub messages: Arc<Mutex<Vec<Message>>>,
    pub chatbot: Arc<Chatbot>,
    pub provider: Arc<dyn ProviderTrait + Send + Sync>,
    pub runtime: Runtime,
    pub is_processing: Arc<AtomicBool>,
    pub ui_sender: mpsc::UnboundedSender<(String, bool)>,
    pub ui_receiver: Arc<Mutex<mpsc::UnboundedReceiver<(String, bool)>>>,
    pub history_manager: Arc<Mutex<ChatHistory>>,
    pub needs_naming: Arc<Mutex<bool>>,
    pub name_sender: mpsc::UnboundedSender<String>,
    pub name_receiver: Arc<Mutex<mpsc::UnboundedReceiver<String>>>,
    pub current_model: Arc<Mutex<String>>,
    pub error_sender: mpsc::UnboundedSender<String>,
    pub error_receiver: Arc<Mutex<mpsc::UnboundedReceiver<String>>>,
    pub has_updates: Arc<Mutex<bool>>,
    pub stop_flag: Arc<AtomicBool>,
    pub current_response: Arc<Mutex<String>>,
    pub message_view: Arc<Mutex<MessageView>>,
}

impl Chat {
    pub fn new(initial_provider: Arc<dyn ProviderTrait + Send + Sync>) -> Self {
        let (ui_sender, ui_receiver) = mpsc::unbounded_channel();
        let (name_sender, name_receiver) = mpsc::unbounded_channel();
        let (error_sender, error_receiver) = mpsc::unbounded_channel();
        let initial_model = initial_provider.models()[0].0.to_string();
        Self {
            messages: Arc::new(Mutex::new(Vec::new())),
            chatbot: Arc::new(Chatbot::new(Arc::clone(&initial_provider))),
            runtime: Runtime::new().unwrap(),
            is_processing: Arc::new(AtomicBool::new(false)),
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
            stop_flag: Arc::new(AtomicBool::new(false)),
            current_response: Arc::new(Mutex::new(String::new())),
            message_view: Arc::new(Mutex::new(MessageView::new())),
        }
    }

    pub fn clear_syntax_cache(&self) {
        self.message_view.lock().unwrap().clear_syntax_cache();
    }

    pub fn update_provider(&mut self, new_provider: Arc<dyn ProviderTrait + Send + Sync>) {
        self.provider = new_provider;
        self.set_has_updates();
    }

    pub fn get_messages(&self) -> Vec<Message> {
        self.messages.lock().unwrap().clone()
    }

    pub fn is_processing(&self) -> bool {
        self.is_processing.load(Ordering::SeqCst)
    }

    pub fn process_input(&self, input: String, model: String) {
        let input_with_newlines = input.replace("\n", "\n").trim().to_string();
        self.add_message(input_with_newlines.clone(), true);
        self.is_processing.store(true, Ordering::SeqCst);
        self.stop_flag.store(false, Ordering::SeqCst);
        let chatbot = Arc::clone(&self.chatbot);
        let is_processing = Arc::clone(&self.is_processing);
        let ui_sender = self.ui_sender.clone();
        let needs_naming = Arc::clone(&self.needs_naming);
        let current_model = Arc::clone(&self.current_model);
        let error_sender = self.error_sender.clone();
        let stop_flag = Arc::clone(&self.stop_flag);
        let provider = Arc::clone(&self.provider);
        let messages = Arc::clone(&self.messages);
        let current_response = Arc::clone(&self.current_response);
    
        self.runtime.spawn(async move {
            provider.set_current_model(model.clone());
            let messages_clone = messages.lock().unwrap().clone();
            match chatbot.stream_response(&messages_clone) {
                Ok(mut rx) => {
                    let mut full_response = String::new();
                    while let Some(chunk) = rx.recv().await {
                        if stop_flag.load(Ordering::SeqCst) {
                            break;
                        }
                        full_response.push_str(&chunk);
                        *current_response.lock().unwrap() = full_response.clone();
                        if ui_sender.send((chunk, false)).is_err() {
                            break;
                        }
                    }

                    let _ = ui_sender.send((full_response.clone(), true));
                    *current_response.lock().unwrap() = String::new();

                    if messages_clone.len() == 1 {
                        *needs_naming.lock().unwrap() = true;
                    }

                    *current_model.lock().unwrap() = model;
                }
                Err(e) => {
                    let error_message = format!("Error: {}", e);
                    error_sender.send(error_message).unwrap();
                }
            }
            is_processing.store(false, Ordering::SeqCst);
        });
        self.set_has_updates();
    }

    pub fn stop_processing(&self) {
        self.stop_flag.store(true, Ordering::SeqCst);
        self.is_processing.store(false, Ordering::SeqCst);
    }

    pub fn generate_chat_name(&self) {
        let chatbot = Arc::clone(&self.chatbot);
        let messages = Arc::clone(&self.messages);
        let name_sender = self.name_sender.clone();
        let needs_naming = Arc::clone(&self.needs_naming);

        chat_name_generation::generate_chat_name(chatbot, messages, name_sender, &self.runtime);
        *needs_naming.lock().unwrap() = false;
    }

    pub fn update_profile(&self, profile: ProfileType) {
        self.provider.update_profile(profile);
    }

}