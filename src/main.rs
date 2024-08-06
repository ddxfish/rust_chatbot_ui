mod chat_bot;
mod api;

use chat_bot::ChatBot;
use iced::{Settings, Application};
use log::info;

fn main() -> iced::Result {
    env_logger::init();
    info!("Starting ChatBot application");
    ChatBot::run(Settings::default())
}