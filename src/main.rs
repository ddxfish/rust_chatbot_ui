mod app;
mod chat;
mod ui;
mod message;
mod chatbot;
mod settings;
mod chat_history;
use eframe::NativeOptions;

fn main() -> Result<(), eframe::Error> {
    let options = NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([850.0, 600.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Rusty Chatbot UI",
        options,
        Box::new(|cc| Ok(Box::new(app::ChatbotApp::new(cc))))
    )
}