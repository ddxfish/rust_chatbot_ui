mod app_state;
mod app_ui;
mod app_initialization;
mod app_updates;
mod icons;

use crate::chat::Chat;
use crate::ui::ChatbotUi;
use crate::settings::Settings;
use crate::providers::{self, Provider};
use eframe;
use std::sync::Arc;
use std::time::Instant;
pub use icons::Icons;
pub use app_state::ChatbotAppState;

pub struct ChatbotApp {
    state: ChatbotAppState,
    chat: Chat,
    ui: ChatbotUi,
    settings: Settings,
    icons: Icons,
    providers: Vec<Arc<dyn Provider + Send + Sync>>,
    theme: crate::ui::themes::Theme,
    last_scale_change: Instant,
}

impl ChatbotApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        app_initialization::initialize_app(cc)
    }

    fn create_providers(api_keys: &str) -> Vec<Arc<dyn Provider + Send + Sync>> {
        providers::get_providers(api_keys.to_string())
            .into_iter()
            .map(|p| Arc::from(p) as Arc<dyn Provider + Send + Sync>)
            .collect()
    }

    fn switch_provider(&mut self, model: String) {
        app_updates::switch_provider(self, model);
    }
}

impl eframe::App for ChatbotApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        app_updates::update(self, ctx);
    }
}