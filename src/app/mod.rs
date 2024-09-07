mod icons;
mod state;

use crate::chat::Chat;
use crate::ui::ChatbotUi;
use crate::settings::Settings;
use crate::providers::{self, Provider};
use crate::ui::themes::Theme;
use eframe;
use std::sync::Arc;
use std::time::{Instant, Duration};
use eframe::egui::{self, FontData, FontDefinitions, FontFamily, Align, Layout, Key, Modifiers};
pub use icons::Icons;
pub use state::ChatbotAppState;

pub struct ChatbotApp {
    state: ChatbotAppState,
    chat: Chat,
    ui: ChatbotUi,
    settings: Settings,
    icons: Icons,
    providers: Vec<Arc<dyn Provider + Send + Sync>>,
    theme: Theme,
    last_scale_change: Instant,
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
        let mut settings = Settings::new();
        let theme = settings.get_current_theme().clone();
        cc.egui_ctx.set_visuals(theme.apply_to_visuals());
        settings.ui_scale = cc.egui_ctx.pixels_per_point();
        cc.egui_ctx.set_pixels_per_point(settings.ui_scale);

        let providers = Self::create_providers(&settings.get_api_keys());
        let initial_provider = settings.get_first_provider_with_key(&providers);
        let initial_model = initial_provider.models()[0].to_string();

        let mut chat = Chat::new(Arc::clone(&initial_provider));
        chat.load_most_recent_or_create_new().unwrap_or_else(|e| eprintln!("Failed to load or create chat: {}", e));

        Self {
            state: ChatbotAppState::new(),
            chat,
            ui: ChatbotUi::new(initial_provider.name().to_string(), initial_model),
            settings,
            icons: Icons::new(&cc.egui_ctx),
            providers,
            theme,
            last_scale_change: Instant::now() - Duration::from_secs(1),
        }
    }

    fn create_providers(api_keys: &str) -> Vec<Arc<dyn Provider + Send + Sync>> {
        providers::get_providers(api_keys.to_string())
            .into_iter()
            .map(|p| Arc::from(p) as Arc<dyn Provider + Send + Sync>)
            .collect()
    }

    fn switch_provider(&mut self, model: String) {
        let (provider, is_custom) = if model.starts_with("accounts/fireworks/models/") {
            (self.providers.iter().find(|p| p.name() == "Fireworks"), true)
        } else {
            (self.providers.iter().find(|p| p.models().contains(&model.as_str())), false)
        };

        if let Some(current_provider) = provider {
            println!("Switching to provider: {} with model: {}", current_provider.name(), model);

            self.chat.update_provider(Arc::clone(current_provider));

            let model_clone = model.clone();
            let providers_clone = self.providers.clone();
            if let Some(chatbot) = Arc::get_mut(&mut self.chat.chatbot) {
                chatbot.switch_model(&providers_clone, model_clone);
            }

            if let Ok(mut current_model) = self.chat.current_model.lock() {
                *current_model = model;
            }

            println!("Provider and model updated successfully");
        } else {
            println!("Error: No provider found for model: {}", model);
        }
    }

    fn reload_providers(&mut self) {
        let api_keys = self.settings.get_api_keys();
        self.providers = Self::create_providers(&api_keys);

        self.ui.selected_provider = self.providers[0].name().to_string();
        self.ui.selected_model = self.providers[0].models()[0].to_string();
        self.chat.update_provider(Arc::clone(&self.providers[0]));
        if let Err(e) = self.chat.create_new_chat() {
            eprintln!("Failed to create new chat: {}", e);
        }
    }

    fn change_ui_scale(&mut self, ctx: &egui::Context, increase: bool) {
        let now = Instant::now();
        if now.duration_since(self.last_scale_change) > Duration::from_millis(200) {
            if increase {
                self.settings.ui_scale = (self.settings.ui_scale + 0.1).min(2.0);
            } else {
                self.settings.ui_scale = (self.settings.ui_scale - 0.1).max(0.5);
            }
            ctx.set_pixels_per_point(self.settings.ui_scale);
            self.last_scale_change = now;
        }
    }
}

impl eframe::App for ChatbotApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        if self.settings.api_keys_updated {
            self.reload_providers();
            self.settings.api_keys_updated = false;
        }

        let current_theme = self.settings.get_current_theme().clone();
        if self.theme.name != current_theme.name {
            self.theme = current_theme;
            ctx.set_visuals(self.theme.apply_to_visuals());
        }

        if ctx.input(|i| i.key_pressed(Key::Minus) && i.modifiers.ctrl) {
            self.change_ui_scale(ctx, false);
        }

        if ctx.input(|i| i.key_pressed(Key::Plus) && i.modifiers.ctrl) {
            self.change_ui_scale(ctx, true);
        }

        let current_pixels_per_point = ctx.pixels_per_point();
        if (current_pixels_per_point - self.settings.ui_scale).abs() > 0.001 {
            self.settings.ui_scale = current_pixels_per_point;
            ctx.set_pixels_per_point(self.settings.ui_scale);
        }

        if self.chat.has_updates() {
            self.state.update(&mut self.chat);
        }

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
        });
    }
}