use crate::chat::Chat;
use crate::ui::ChatbotUi;
use crate::settings::Settings;
use crate::chat_history::ChatHistory;
use eframe;
use eframe::egui::{self, ScrollArea, Color32};

pub struct ChatbotApp {
    chat: Chat,
    ui: ChatbotUi,
    settings: Settings,
    chat_history: ChatHistory,
}

impl ChatbotApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            chat: Chat::new(),
            ui: ChatbotUi::new(),
            settings: Settings::new(),
            chat_history: ChatHistory::new("chat_history"),
        }
    }
}

impl eframe::App for ChatbotApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        eframe::egui::SidePanel::left("chat_history_panel")
            .default_width(250.0) // Increased width by about 50%
            .show(ctx, |ui| {
                if ui.button("New Chat").clicked() {
                    if let Err(e) = self.chat_history.create_new_chat() {
                        eprintln!("Failed to create new chat: {}", e);
                    }
                }
                
                ScrollArea::vertical().show(ui, |ui| {
                    let mut files = self.chat_history.get_history_files().clone();
                    files.sort_by(|a, b| b.cmp(a)); // Sort in descending order (most recent first)
                    
                    for file in files {
                        ui.add(egui::Label::new(egui::RichText::new(&file)
                            .color(Color32::WHITE)
                            .line_height(Some(30.0)))); // Increased line height
                    }
                });
            });

        eframe::egui::CentralPanel::default().show(ctx, |ui| {
            self.ui.render(ui, &mut self.chat, &mut self.settings);
        });
    }
}