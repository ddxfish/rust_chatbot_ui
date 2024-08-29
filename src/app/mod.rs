mod icons;
mod state;

use crate::chat::Chat;
use crate::ui::ChatbotUi;
use crate::settings::Settings;
use crate::providers::{self, Provider};
use crate::ui::theme::DarkTheme;
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
    theme: DarkTheme,
}

fn load_custom_font(ctx: &eframe::egui::Context) {
    let mut fonts = FontDefinitions::default();
    fonts.font_data.insert(
        "NotoSans".to_owned(),
        FontData::from_static(include_bytes!("../../assets/NotoSans-Medium.ttf")),
    );
    fonts.families.get_mut(&FontFamily::Proportional).unwrap()
        .insert(0, "NotoSans".to_owned());
    ctx.set_fonts(fonts);
}

impl ChatbotApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        load_custom_font(&cc.egui_ctx);
        let theme = DarkTheme::new();
        cc.egui_ctx.set_visuals(theme.apply_to_visuals());
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
            theme,
        }
    }

    fn switch_provider(&mut self, model: String) {
        if let Some(current_provider) = self.providers.iter().find(|p| p.models().contains(&model.as_str())) {
            println!("Switching to provider type: {}", std::any::type_name::<dyn Provider + Send + Sync>());
            
            // Update the provider in the existing chat
            self.chat.update_provider(Arc::clone(current_provider));
            
            // Update the selected model in the UI
            let model_clone = model.clone();
            let providers_clone = self.providers.clone();
            if let Some(chatbot) = Arc::get_mut(&mut self.chat.chatbot) {
                chatbot.switch_model(&providers_clone, model_clone);
            }
            
            // Update the current model in the chat
            if let Ok(mut current_model) = self.chat.current_model.lock() {
                *current_model = model;
            }
            
            println!("Provider and model updated successfully");
        }
    }
}

impl eframe::App for ChatbotApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        self.state.update(&mut self.chat);

        eframe::egui::SidePanel::left("chat_history_panel")
            .resizable(true)
            .default_width(200.0)
            .width_range(150.0..=400.0)
            .show(ctx, |ui| {
                ui.with_layout(Layout::top_down_justified(Align::LEFT), |ui| {
                    let available_height = ui.available_height();
                    let bottom_panel_height = 100.0;
                    
                    egui::ScrollArea::vertical().max_height(available_height - bottom_panel_height).show(ui, |ui| {
                        self.state.render_chat_history(ui, &mut self.chat, &self.icons, &self.theme);
                    });

                    ui.with_layout(Layout::bottom_up(Align::LEFT), |ui| {
                        ui.set_min_height(bottom_panel_height);
                        self.state.render_bottom_left_section(ui, &mut self.chat, &mut self.settings, &mut self.ui, &self.providers, &self.theme);
                    });
                });
            });

            eframe::egui::CentralPanel::default().show(ctx, |ui| {
                self.ui.render(ui, &mut self.chat, &mut self.settings, &self.icons, &self.providers, &self.theme);
                
                if let Some(previous_model) = self.state.previous_model.take() {
                    if previous_model != self.ui.selected_model {
                        self.switch_provider(self.ui.selected_model.clone());
                    }
                }
                self.state.previous_model = Some(self.ui.selected_model.clone());
                ctx.request_repaint();
            });

        
    }
}