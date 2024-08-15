use crate::chat::Chat;
use crate::app::Icons;
use eframe::egui::{self, Ui, Layout, Align, RichText, Button, Image, Vec2, Color32};

pub struct ChatHistory {
    selected_chat: Option<String>,
    need_to_load_most_recent: bool,
}

impl ChatHistory {
    pub fn new() -> Self {
        Self {
            selected_chat: None,
            need_to_load_most_recent: true,
        }
    }

    pub fn update(&mut self, chat: &mut Chat) {
        if self.need_to_load_most_recent {
            if let Some(most_recent) = chat.get_history_files().first().cloned() {
                if let Err(e) = chat.load_chat(&most_recent) {
                    eprintln!("Failed to load most recent chat: {}", e);
                }
            } else {
                if let Err(e) = chat.create_new_chat() {
                    eprintln!("Failed to create new chat: {}", e);
                }
            }
            self.need_to_load_most_recent = false;
        }

        if let Some(file) = self.selected_chat.take() {
            if let Err(e) = chat.load_chat(&file) {
                eprintln!("Failed to load chat: {}", e);
            }
        }
    }

    pub fn render(&mut self, ui: &mut Ui, chat: &mut Chat, icons: &Icons) -> Option<String> {
        let mut file_to_delete = None;

        ui.with_layout(Layout::top_down_justified(Align::LEFT), |ui| {
            ui.horizontal(|ui| {
                ui.add(Image::new(&icons.new_chat).fit_to_exact_size(Vec2::new(40.0, 40.0)));
                if ui.button(RichText::new("New Chat").size(20.0)).clicked() {
                    if let Err(e) = chat.create_new_chat() {
                        eprintln!("Failed to create new chat: {}", e);
                    }
                }
            });
        });
        
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.with_layout(Layout::top_down_justified(Align::LEFT), |ui| {
                let files = chat.get_history_files();
                let current_file = chat.get_current_file();
                
                for file in files {
                    ui.horizontal(|ui| {
                        let is_current = current_file.as_ref().map_or(false, |current| current == &file);
                        let display_name = format_file_name(&file);
                        let text = if is_current {
                            RichText::new(display_name).color(Color32::YELLOW).size(18.0)
                        } else {
                            RichText::new(display_name).color(Color32::WHITE).size(18.0)
                        };
                        
                        if ui.add(egui::Label::new(text).wrap()).clicked() {
                            self.selected_chat = Some(file.clone());
                        }
                        
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            if ui.add(Button::image(Image::new(&icons.trash).fit_to_exact_size(Vec2::new(10.0, 10.0)))).clicked() {
                                file_to_delete = Some(file.clone());
                            }
                        });
                    });
                    ui.add_space(5.0);
                }
            });
        });

        file_to_delete
    }
}

fn format_file_name(file_name: &str) -> String {
    file_name
        .trim_end_matches(".txt")
        .replace('_', " ")
}