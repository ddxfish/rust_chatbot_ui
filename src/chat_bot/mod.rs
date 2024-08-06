mod ui;
mod message;

use iced::{executor, Application, Command, Element, Length, Theme};
use serde_json::json;
use tokio::runtime::Runtime;
use log::{debug, error, info};

use crate::api::client::send_message_to_api;
pub use message::Message;
use ui::view;

pub struct ChatBot {
    input: String,
    messages: Vec<String>,
    api_key: String,
    conversation_history: Vec<serde_json::Value>,
    runtime: Runtime,
}

impl Application for ChatBot {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let runtime = Runtime::new().expect("Failed to create Tokio runtime");
        info!("ChatBot initialized");
        (
            Self {
                input: String::new(),
                messages: Vec::new(),
                api_key: String::from("xxx"), // Replace with your actual API key
                conversation_history: Vec::new(),
                runtime,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Rust ChatBot")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::InputChanged(input) => {
                self.input = input;
                debug!("Input changed: {}", self.input);
                Command::none()
            }
            Message::Send | Message::InputSubmit => { // Handle both Send and InputSubmit
                let user_message = self.input.clone();
                self.messages.push(format!("You: {}", user_message));
                self.conversation_history.push(json!({
                    "role": "user",
                    "content": user_message
                }));
                self.input.clear();
                info!("Sending message to API");
    
                let conversation_history = self.conversation_history.clone();
                let api_key = self.api_key.clone();
                let handle = self.runtime.handle().clone();
    
                Command::perform(
                    async move {
                        handle.block_on(send_message_to_api(conversation_history, api_key))
                    },
                    Message::Received,
                )
            }
            Message::Received(Ok(response)) => {
                info!("Received response from API");
                self.messages.push(format!("Bot: {}", response));
                self.conversation_history.push(json!({
                    "role": "assistant",
                    "content": response
                }));
                Command::none()
            }
            Message::Received(Err(error)) => {
                error!("Error received from API: {}", error);
                self.messages.push(format!("Error: {}", error));
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        view(self)
    }
}