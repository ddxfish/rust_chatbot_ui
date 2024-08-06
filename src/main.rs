use iced::widget::{button, column, container, row, scrollable, text, text_input};
use iced::{executor, Alignment, Application, Command, Element, Length, Settings, Theme};
use serde_json::json;
use tokio::runtime::Runtime;
use log::{debug, error, info};

fn main() -> iced::Result {
    env_logger::init();
    ChatBot::run(Settings::default())
}

struct ChatBot {
    input: String,
    messages: Vec<String>,
    api_key: String,
    conversation_history: Vec<serde_json::Value>,
    runtime: Runtime,
}

#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
    Send,
    Received(Result<String, String>),
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
            Message::Send => {
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
        let input = text_input("Type a message...", &self.input)
            .on_input(Message::InputChanged)
            .padding(10);

        let send_button = button("Send").on_press(Message::Send);

        let input_row = row![input, send_button].spacing(10).align_items(Alignment::Center);

        let messages: Element<_> = self
            .messages
            .iter()
            .fold(column![].spacing(10), |column, msg| {
                column.push(text(msg))
            })
            .into();

        let content = column![
            scrollable(container(messages).width(Length::Fill).padding(20))
                .height(Length::Fill),
            input_row
        ]
        .spacing(20)
        .padding(20)
        .align_items(Alignment::Center);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

async fn send_message_to_api(conversation_history: Vec<serde_json::Value>, api_key: String) -> Result<String, String> {
    debug!("Sending message to API");
    let client = reqwest::Client::new();
    let response = client
        .post("https://api.fireworks.ai/inference/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&json!({
            "model": "accounts/fireworks/models/llama-v3-8b-instruct",
            "messages": conversation_history
        }))
        .send()
        .await
        .map_err(|e| {
            error!("API request error: {}", e);
            e.to_string()
        })?;

    let response_json: serde_json::Value = response.json().await.map_err(|e| {
        error!("Failed to parse API response: {}", e);
        e.to_string()
    })?;
    let bot_message = response_json["choices"][0]["message"]["content"]
        .as_str()
        .ok_or("Failed to parse response")?
        .to_string();

    debug!("Received bot message: {}", bot_message);
    Ok(bot_message)
}