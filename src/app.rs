use crate::chat::Chat;
use crate::ui::ChatbotUi;
use crate::settings::Settings;
use eframe;
use eframe::egui::{self, ScrollArea, Color32, Layout, Align};

pub struct ChatbotApp {
    chat: Chat,
    ui: ChatbotUi,
    settings: Settings,
    left_panel_width: f32,
    selected_chat: Option<String>,
}

impl ChatbotApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        cc.egui_ctx.set_pixels_per_point(1.0);
        
        Self {
            chat: Chat::new(),
            ui: ChatbotUi::new(),
            settings: Settings::new(),
            left_panel_width: 200.0,
            selected_chat: None,
        }
    }
}

impl eframe::App for ChatbotApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        if let Some(file) = self.selected_chat.take() {
            if let Err(e) = self.chat.load_chat(&file) {
                eprintln!("Failed to load chat: {}", e);
            }
        }

        eframe::egui::SidePanel::left("chat_history_panel")
            .resizable(true)
            .default_width(200.0)
            .width_range(100.0..=400.0)
            .show(ctx, |ui| {
                ui.with_layout(Layout::top_down_justified(Align::LEFT), |ui| {
                    if ui.button("New Chat").clicked() {
                        if let Err(e) = self.chat.create_new_chat() {
                            eprintln!("Failed to create new chat: {}", e);
                        }
                    }
                });
                
                ScrollArea::vertical().show(ui, |ui| {
                    ui.with_layout(Layout::top_down_justified(Align::LEFT), |ui| {
                        let files = self.chat.get_history_files();
                        let current_file = self.chat.get_current_file().cloned();
                        
                        for file in files {
                            let is_current = current_file.as_ref().map_or(false, |current| current == file);
                            let text = if is_current {
                                egui::RichText::new(file).color(Color32::YELLOW)
                            } else {
                                egui::RichText::new(file).color(Color32::WHITE)
                            };
                            
                            if ui.add(egui::Label::new(text).wrap()).clicked() {
                                self.selected_chat = Some(file.clone());
                            }
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