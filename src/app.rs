use crate::chat::Chat;
use crate::ui::ChatbotUi;
use crate::settings::Settings;
use eframe;

pub struct ChatbotApp {
    chat: Chat,
    ui: ChatbotUi,
    settings: Settings,
}

impl ChatbotApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            chat: Chat::new(),
            ui: ChatbotUi::new(),
            settings: Settings::new(),
        }
    }
}

impl eframe::App for ChatbotApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        eframe::egui::CentralPanel::default().show(ctx, |ui| {
            self.settings.render(ui);
            self.ui.render(ui, &mut self.chat);
        });
    }
}