mod app;
mod chat;
mod ui;
mod message;
mod chatbot;

use eframe::NativeOptions;

fn main() -> Result<(), eframe::Error> {
    let options = NativeOptions::default();
    eframe::run_native(
        "Rust Chatbot",
        options,
        Box::new(|cc| Box::new(app::ChatbotApp::new(cc)))
    )
}