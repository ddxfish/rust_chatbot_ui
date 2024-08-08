//use egui::Context;
use crate::chat::Chat;
use crate::ui::ChatbotUi;
use eframe;
pub struct ChatbotApp {
    chat: Chat,
    ui: ChatbotUi,
}

impl ChatbotApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            chat: Chat::new(),
            ui: ChatbotUi::new(),
        }
    }
}

impl eframe::App for ChatbotApp {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        eframe::egui::CentralPanel::default().show(ctx, |ui: &mut eframe::egui::Ui| {
            self.ui.render(ui, &mut self.chat);
        });
    }
}