use crate::chat::Chat;
use crate::app::Icons;
use crate::themes::Theme;
use eframe::egui::{self, RichText, Button, Image, Vec2, Layout, Align};

pub fn render_history(ui: &mut egui::Ui, chat: &mut Chat, icons: &Icons, theme: &Theme, history_files: &[String], current_file: Option<&String>) {
    ui.with_layout(Layout::top_down_justified(Align::LEFT), |ui| {
        ui.horizontal(|ui| {
            ui.add(Image::new(&icons.new_chat).fit_to_exact_size(Vec2::new(40.0, 40.0)));
            if ui.add(Button::new(RichText::new("New Chat").size(20.0).color(theme.new_chat_button_text_color)).fill(theme.button_bg_color)).clicked() {
                if let Err(e) = chat.create_new_chat() {
                    eprintln!("Failed to create new chat: {}", e);
                }
            }
        });
    });

    egui::ScrollArea::vertical().show(ui, |ui| {
        ui.with_layout(Layout::top_down_justified(Align::LEFT), |ui| {
            for file in history_files {
                ui.horizontal(|ui| {
                    let is_current = current_file.map_or(false, |current| current == file);
                    let display_name = format_file_name(file);
                    let text = if is_current {
                        RichText::new(display_name).color(theme.selected_chat_color).size(18.0)
                    } else {
                        RichText::new(display_name).color(theme.unselected_chat_color).size(18.0)
                    };

                    if ui.add(egui::Label::new(text).wrap()).clicked() {
                        if let Err(e) = chat.load_chat(file) {
                            eprintln!("Failed to load chat: {}", e);
                        }
                    }

                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        if ui.add(Button::image(Image::new(&icons.trash).fit_to_exact_size(Vec2::new(10.0, 10.0))).fill(theme.trash_button_bg_color)).clicked() {
                            if let Err(e) = chat.delete_chat(file) {
                                eprintln!("Failed to delete chat: {}", e);
                            }
                        }
                    });
                });
                ui.add_space(5.0);
            }
        });
    });
}

pub fn format_file_name(file_name: &str) -> String {
    file_name
        .trim_end_matches(".txt")
        .replace('_', " ")
}