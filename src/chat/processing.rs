use crate::message::Message;
use crate::chatbot::Chatbot;
use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;
use tokio::sync::mpsc;

pub struct Processing;

impl Processing {
    pub fn new() -> Self {
        Self
    }

    pub fn process_input(
        &self,
        input: String,
        chatbot: &Arc<Chatbot>,
        messages: &Arc<Mutex<Vec<Message>>>,
        is_processing: &Arc<Mutex<bool>>,
        ui_sender: &mpsc::UnboundedSender<String>,
        runtime: &Runtime,
    ) {
        println!("Processing input: {}", input);
        *is_processing.lock().unwrap() = true;
        let chatbot = Arc::clone(chatbot);
        let messages = Arc::clone(messages);
        let is_processing = Arc::clone(is_processing);
        let ui_sender = ui_sender.clone();

        runtime.spawn(async move {
            println!("Starting to generate response");
            let messages = messages.lock().unwrap().clone();
            match chatbot.stream_response(&messages).await {
                Ok(mut rx) => {
                    println!("Response stream received");
                    while let Some(chunk) = rx.recv().await {
                        println!("Received chunk: {}", chunk);
                        if ui_sender.send(chunk).is_err() {
                            eprintln!("Failed to send chunk to UI");
                            break;
                        }
                    }
                },
                Err(e) => {
                    eprintln!("Error generating response: {}", e);
                }
            }
            *is_processing.lock().unwrap() = false;
            println!("Finished processing");
        });
    }

    pub fn check_ui_updates(&self, ui_receiver: &Arc<Mutex<mpsc::UnboundedReceiver<String>>>) -> Option<String> {
        ui_receiver.lock().unwrap().try_recv().ok()
    }
}