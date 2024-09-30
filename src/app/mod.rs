mod app_state;
mod app_ui;
mod app_initialization;
mod app_updates;
mod icons;

use crate::chat::Chat;
use crate::ui::ChatbotUi;
use crate::settings::Settings;
use crate::providers::{self, ProviderTrait};
use eframe;
use std::sync::Arc;
use std::time::Instant;
pub use icons::Icons;
pub use app_state::ChatbotAppState;
use egui::Color32;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ProfileType {
    Coder,
    Normal,
    Creative,
}

pub struct ChatbotApp {
    state: ChatbotAppState,
    chat: Chat,
    ui: ChatbotUi,
    settings: Settings,
    icons: Icons,
    providers: Vec<Arc<dyn ProviderTrait + Send + Sync>>,
    theme: crate::ui::themes::Theme,
    last_scale_change: Instant,
    current_profile: ProfileType,
    pub bot_text_color: Color32,
    pub user_text_color: Color32,
}

impl ChatbotApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut app = app_initialization::initialize_app(cc);
        
        // Initialize text colors from the current theme
        app.bot_text_color = app.theme.bot_text_color;
        app.user_text_color = app.theme.user_text_color;

        app
    }

    fn create_providers(api_keys: &str) -> Vec<Arc<dyn ProviderTrait + Send + Sync>> {
        providers::get_providers(api_keys.to_string())
            .into_iter()
            .map(|p| Arc::from(p) as Arc<dyn ProviderTrait + Send + Sync>)
            .collect()
    }

    fn switch_provider(&mut self, model: String) {
        app_updates::switch_provider(self, model);
    }

    pub fn update_profile(&mut self, profile: ProfileType) {
        self.current_profile = profile;
        if let Some(provider) = self.providers.iter().find(|p| p.name() == self.ui.selected_provider) {
            provider.update_profile(profile);
        }
        self.chat.update_profile(profile);
    }
}

impl eframe::App for ChatbotApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        app_updates::update(self, ctx);
    }
}