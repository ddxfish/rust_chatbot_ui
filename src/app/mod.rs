mod icons;
mod state;

use crate::chat::Chat;
use crate::ui::ChatbotUi;
use crate::settings::Settings;
use crate::providers::{self, Provider};
use eframe;
use std::sync::Arc;

pub use icons::Icons;
pub use state::ChatbotAppState;

pub struct ChatbotApp {
    state: ChatbotAppState,
    chat: Chat,
    ui: ChatbotUi,
    settings: Settings,
    icons: Icons,
}

impl ChatbotApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        cc.egui_ctx.set_pixels_per_point(1.0);
        
        let settings = Settings::new();
        let providers: Vec<Arc<dyn Provider + Send + Sync>> = providers::get_providers(settings.get_api_keys())
        .into_iter()
        .map(|p| Arc::from(p) as Arc<dyn Provider + Send + Sync>)
        .collect();
        
        let chat = Chat::new(Arc::clone(&providers[0]));
        
        Self {
            state: ChatbotAppState::new(),
            chat,
            ui: ChatbotUi::new(),
            settings,
            icons: Icons::new(&cc.egui_ctx),
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
                self.state.render_chat_history(ui, &mut self.chat, &self.icons);
            });

        eframe::egui::CentralPanel::default().show(ctx, |ui| {
            self.ui.render(ui, &mut self.chat, &mut self.settings, &self.icons);
            ctx.request_repaint();
        });

        self.state.handle_delete_confirmation(ctx, &mut self.chat);
    }
}