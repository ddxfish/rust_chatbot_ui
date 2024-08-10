use crate::chat::Chat;
use crate::ui::ChatbotUi;
use crate::settings::Settings;
use crate::chat_history::ChatHistory;
use eframe;
use eframe::egui::{self, ScrollArea, Color32, Layout, Align};

pub struct ChatbotApp {
    chat: Chat,
    ui: ChatbotUi,
    settings: Settings,
    chat_history: ChatHistory,
    left_panel_width: f32,
}

impl ChatbotApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        cc.egui_ctx.set_pixels_per_point(1.0);
        
        Self {
            chat: Chat::new(),
            ui: ChatbotUi::new(),
            settings: Settings::new(),
            chat_history: ChatHistory::new("chat_history"),
            left_panel_width: 200.0,
        }
    }
}

impl eframe::App for ChatbotApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        eframe::egui::SidePanel::left("chat_history_panel")
            .resizable(true)
            .default_width(200.0)
            .width_range(100.0..=400.0)
            .show(ctx, |ui| {
                ui.with_layout(Layout::top_down_justified(Align::LEFT), |ui| {
                    if ui.button("New Chat").clicked() {
                        if let Err(e) = self.chat_history.create_new_chat() {
                            eprintln!("Failed to create new chat: {}", e);
                        }
                    }
                });
                
                ScrollArea::vertical().show(ui, |ui| {
                    ui.with_layout(Layout::top_down_justified(Align::LEFT), |ui| {
                        let mut files = self.chat_history.get_history_files().clone();
                        files.sort_by(|a, b| b.cmp(a));
                        
                        for file in files {
                            ui.add(egui::Label::new(egui::RichText::new(&file)
                                .color(Color32::WHITE))
                                .wrap());
                            ui.add_space(5.0);
                        }
                    });
                });
            });

        eframe::egui::CentralPanel::default().show(ctx, |ui| {
            self.ui.render(ui, &mut self.chat, &mut self.settings);
        });
    }
}