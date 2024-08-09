mod app;
mod chat;
mod ui;
mod message;
mod chatbot;
mod settings;
use eframe::NativeOptions;

fn main() -> Result<(), eframe::Error> {
    let options = NativeOptions::default();
    eframe::run_native(
        "Rusty Chatbot UI",
        options,
        Box::new(|cc| Ok(Box::new(app::ChatbotApp::new(cc))))
    )
}