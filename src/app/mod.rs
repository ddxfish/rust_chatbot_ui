mod icons;
mod state;

use crate::chat::Chat;
use crate::ui::ChatbotUi;
use crate::settings::Settings;
use crate::providers::{self, Provider};
use eframe;
use std::sync::Arc;
use eframe::egui::{self, FontData, FontDefinitions, FontFamily, Align, Layout};
pub use icons::Icons;
pub use state::ChatbotAppState;

pub struct ChatbotApp {
    state: ChatbotAppState,
    chat: Chat,
    ui: ChatbotUi,
    settings: Settings,
    icons: Icons,
    providers: Vec<Arc<dyn Provider + Send + Sync>>,
    current_provider_index: usize,
}

fn load_custom_font(ctx: &eframe::egui::Context) {
    let mut fonts = FontDefinitions::default();
    fonts.font_data.insert(
        "Roboto".to_owned(),
        FontData::from_static(include_bytes!("../../assets/Lora-Regular.ttf")),
    );
    fonts.families.get_mut(&FontFamily::Proportional).unwrap()
        .insert(0, "Roboto".to_owned());
    ctx.set_fonts(fonts);
}

impl ChatbotApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        load_custom_font(&cc.egui_ctx);
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        cc.egui_ctx.set_pixels_per_point(1.0);
        
        let settings = Settings::new();
        let providers: Vec<Arc<dyn Provider + Send + Sync>> = providers::get_providers(settings.get_api_keys())
            .into_iter()
            .map(|p| Arc::from(p) as Arc<dyn Provider + Send + Sync>)
            .collect();
        
        let initial_provider = providers[1].name().to_string();
        let initial_model = providers[1].models()[0].to_string();
        let chat = Chat::new(Arc::clone(&providers[1]));
        
        Self {
            state: ChatbotAppState::new(),
            chat,
            ui: ChatbotUi::new(initial_provider, initial_model),
            settings,
            icons: Icons::new(&cc.egui_ctx),
            providers,
            current_provider_index: 1,
        }
    }

    fn switch_provider(&mut self, index: usize) {
        if index < self.providers.len() && index != self.current_provider_index {
            let current_file = self.chat.get_current_file().map(String::from);
            
            self.current_provider_index = index;
            self.chat = Chat::new(Arc::clone(&self.providers[index]));
            
            if let Some(file) = current_file {
                if let Err(e) = self.chat.load_chat(&file) {
                    eprintln!("Failed to load chat after switching provider: {}", e);
                }
            }

            self.ui.selected_provider = self.providers[index].name().to_string();
            self.ui.selected_model = self.providers[index].models()[0].to_string();
        }
    }
}

impl eframe::App for ChatbotApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        self.state.update(&mut self.chat);

        eframe::egui::SidePanel::left("chat_history_panel")
            .resizable(true)
            .default_width(200.0)
            .width_range(100.0..=400.0)
            .show(ctx, |ui| {
                ui.with_layout(Layout::top_down(Align::LEFT), |ui| {
                    let available_height = ui.available_height();
                    let bottom_panel_height = 100.0; // Reduced height
                    
                    self.state.render_chat_history(ui, &mut self.chat, &self.icons);

                    ui.allocate_space(egui::vec2(ui.available_width(), available_height - bottom_panel_height - ui.min_size().y));

                    ui.with_layout(Layout::bottom_up(Align::LEFT), |ui| {
                        ui.set_max_height(bottom_panel_height);
                        self.state.render_bottom_left_section(ui, &mut self.chat, &mut self.settings, &mut self.ui, &self.providers);
                    });
                });
            });

        eframe::egui::CentralPanel::default().show(ctx, |ui| {
            let previous_provider = self.ui.selected_provider.clone();
            self.ui.render(ui, &mut self.chat, &mut self.settings, &self.icons, &self.providers);
            
            if previous_provider != self.ui.selected_provider {
                let new_index = self.providers.iter().position(|p| p.name() == self.ui.selected_provider).unwrap_or(0);
                self.switch_provider(new_index);
            }
            
            ctx.request_repaint();
        });

        self.state.handle_delete_confirmation(ctx, &mut self.chat);
    }
}
