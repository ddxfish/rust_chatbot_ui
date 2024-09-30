use crate::app::{ChatbotApp, ChatbotAppState, Icons};
use crate::chat::Chat;
use crate::ui::ChatbotUi;
use crate::settings::Settings;
use crate::app::ProfileType;
use eframe;
use std::sync::Arc;
use eframe::egui::{FontData, FontDefinitions, FontFamily};

pub fn initialize_app(cc: &eframe::CreationContext<'_>) -> ChatbotApp {
    load_custom_font(&cc.egui_ctx);
    let mut settings = Settings::new();
    let theme = settings.get_current_theme().clone();
    cc.egui_ctx.set_visuals(theme.apply_to_visuals());
    settings.ui_scale = cc.egui_ctx.pixels_per_point();
    cc.egui_ctx.set_pixels_per_point(settings.ui_scale);

    let providers = ChatbotApp::create_providers(&settings.get_api_keys());
    let initial_provider = settings.get_first_provider_with_key(&providers);
    let initial_model = initial_provider.models()[0].to_string();

    let chat = Chat::new(Arc::clone(&initial_provider));
    chat.load_most_recent_or_create_new().unwrap_or_else(|e| eprintln!("Failed to load or create chat: {}", e));

    ChatbotApp {
        state: ChatbotAppState::new(),
        chat,
        ui: ChatbotUi::new(initial_provider.name().to_string(), initial_model),
        settings,
        icons: Icons::new(&cc.egui_ctx),
        providers,
        theme: theme.clone(),
        last_scale_change: std::time::Instant::now() - std::time::Duration::from_secs(1),
        current_profile: ProfileType::Normal,
        bot_text_color: theme.bot_text_color,
        user_text_color: theme.user_text_color,
    }
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