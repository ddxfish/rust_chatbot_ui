use egui::Context;
use crate::chat::Chat;
use crate::ui::ChatbotUi;

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
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.ui.render(ui, &mut self.chat);
        });
    }
}