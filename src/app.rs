use crate::chat::Chat;
use crate::ui::ChatbotUi;
use crate::settings::Settings;
use crate::providers::{self, Provider};
use eframe;
use eframe::egui::{self, ScrollArea, Color32, Layout, Align, TextureHandle, Image, Vec2, Button, RichText};
use std::sync::Arc;

pub struct Icons {
    pub send: TextureHandle,
    pub close: TextureHandle,
    pub new_chat: TextureHandle,
    pub trash: TextureHandle,
}

impl Icons {
    fn new(ctx: &egui::Context) -> Self {
        Self {
            send: load_icon_texture(ctx, "assets/send_icon.png"),
            close: load_icon_texture(ctx, "assets/close_icon.png"),
            new_chat: load_icon_texture(ctx, "assets/app_icon.png"),
            trash: load_icon_texture(ctx, "assets/trash_icon.png"),
        }
    }
}

fn load_icon_texture(ctx: &egui::Context, path: &str) -> TextureHandle {
    let image = image::open(path).expect("Failed to open icon").into_rgba8();
    let (width, height) = image.dimensions();
    ctx.load_texture(
        path,
        egui::ColorImage::from_rgba_unmultiplied([width as _, height as _], &image),
        egui::TextureOptions::default()
    )
}

pub struct ChatbotApp {
    chat: Chat,
    ui: ChatbotUi,
    settings: Settings,
    icons: Icons,
    delete_confirmation: Option<String>,
    selected_chat: Option<String>,
    providers: Vec<Arc<dyn Provider + Send + Sync>>,
    selected_provider: usize,
    need_to_load_most_recent: bool,
}

impl ChatbotApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        cc.egui_ctx.set_pixels_per_point(1.0);
        
        let settings = Settings::new();
        let providers: Vec<Arc<dyn Provider + Send + Sync>> = providers::get_providers(settings.get_fireworks_api_key().to_string())
            .into_iter()
            .map(|p| Arc::from(p) as Arc<dyn Provider + Send + Sync>)
            .collect();
        
        let chat = Chat::new(Arc::clone(&providers[0]));
        
        Self {
            chat,
            ui: ChatbotUi::new(),
            settings,
            icons: Icons::new(&cc.egui_ctx),
            delete_confirmation: None,
            selected_chat: None,
            providers,
            selected_provider: 0,
            need_to_load_most_recent: false,
        }
    }
}

impl eframe::App for ChatbotApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        if self.need_to_load_most_recent {
            if let Some(most_recent) = self.chat.get_history_files().first().cloned() {
                if let Err(e) = self.chat.load_chat(&most_recent) {
                    eprintln!("Failed to load most recent chat: {}", e);
                }
            } else {
                // If no chats left, create a new one
                if let Err(e) = self.chat.create_new_chat() {
                    eprintln!("Failed to create new chat: {}", e);
                }
            }
            self.need_to_load_most_recent = false;
        }

        if let Some(file) = self.selected_chat.take() {
            if let Err(e) = self.chat.load_chat(&file) {
                eprintln!("Failed to load chat: {}", e);
            }
        }

        let mut delete_requested: Option<String> = None;
        let mut delete_confirmed: Option<String> = None;

        eframe::egui::SidePanel::left("chat_history_panel")
            .resizable(true)
            .default_width(200.0)
            .width_range(100.0..=400.0)
            .show(ctx, |ui| {
                ui.with_layout(Layout::top_down_justified(Align::LEFT), |ui| {
                    ui.horizontal(|ui| {
                        ui.add(Image::new(&self.icons.new_chat).fit_to_exact_size(Vec2::new(40.0, 40.0)));
                        if ui.button(RichText::new("New Chat").size(20.0)).clicked() {
                            if let Err(e) = self.chat.create_new_chat() {
                                eprintln!("Failed to create new chat: {}", e);
                            }
                        }
                    });
                });
                
                ScrollArea::vertical().show(ui, |ui| {
                    ui.with_layout(Layout::top_down_justified(Align::LEFT), |ui| {
                        let files = self.chat.get_history_files();
                        let current_file = self.chat.get_current_file().cloned();
                        
                        for file in files {
                            ui.horizontal(|ui| {
                                let is_current = current_file.as_ref().map_or(false, |current| current == file);
                                let text = if is_current {
                                    egui::RichText::new(file.clone()).color(Color32::YELLOW).size(18.0)
                                } else {
                                    egui::RichText::new(file.clone()).color(Color32::WHITE).size(18.0)
                                };
                                
                                if ui.add(egui::Label::new(text).wrap()).clicked() {
                                    self.selected_chat = Some(file.clone());
                                }
                                
                                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                    if ui.add(Button::image(Image::new(&self.icons.trash).fit_to_exact_size(Vec2::new(10.0, 10.0)))).clicked() {
                                        delete_requested = Some(file.clone());
                                    }
                                });
                            });
                            ui.add_space(5.0);
                        }
                    });
                });
            });

        eframe::egui::CentralPanel::default().show(ctx, |ui| {
            self.ui.render(ui, &mut self.chat, &mut self.settings, &self.icons);
        });

        if let Some(file_to_delete) = &self.delete_confirmation {
            egui::Window::new("Confirm Delete")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.label(format!("Are you sure you want to delete '{}'?", file_to_delete));
                    ui.horizontal(|ui| {
                        if ui.button("Yes").clicked() {
                            delete_confirmed = Some(file_to_delete.clone());
                        }
                        if ui.button("No").clicked() {
                            delete_confirmed = Some(String::new()); // Use empty string to signal cancellation
                        }
                    });
                });
        }

        if let Some(file) = delete_requested {
            self.delete_confirmation = Some(file);
        }

        if let Some(file) = delete_confirmed {
            if !file.is_empty() {
                let current_file = self.chat.get_current_file().cloned();
                if let Err(e) = self.chat.delete_chat(&file) {
                    eprintln!("Failed to delete chat: {}", e);
                } else {
                    // If the deleted chat was the current one, set flag to load the most recent chat
                    if Some(&file) == current_file.as_ref() {
                        self.need_to_load_most_recent = true;
                    }
                }
            }
            self.delete_confirmation = None;
        }
    }
}