use crate::chat::Chat;
use crate::ui::ChatbotUi;
use crate::settings::Settings;
use crate::chat_history::ChatHistory;
use eframe;

pub struct ChatbotApp {
    chat: Chat,
    ui: ChatbotUi,
    settings: Settings,
    chat_history: ChatHistory,
}

impl ChatbotApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut chat_history = ChatHistory::new();
        chat_history.load_history("chat_history");
        Self {
            chat: Chat::new(),
            ui: ChatbotUi::new(),
            settings: Settings::new(),
            chat_history,
        }
    }
}

impl eframe::App for ChatbotApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        eframe::egui::SidePanel::left("chat_history_panel").show(ctx, |ui| {
            ui.heading("Chat History");
            for file in self.chat_history.get_history_files() {
                ui.label(file);
            }
        });

        eframe::egui::CentralPanel::default().show(ctx, |ui| {
            self.ui.render(ui, &mut self.chat, &mut self.settings);
        });
    }
}