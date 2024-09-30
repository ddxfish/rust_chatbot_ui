use crate::app::ChatbotApp;
use eframe::egui::{self, Key};
use std::sync::Arc;
use std::time::Instant;

pub fn update(app: &mut ChatbotApp, ctx: &egui::Context) {
    if app.settings.api_keys_updated {
        reload_providers(app);
        app.settings.api_keys_updated = false;
    }

    let current_theme = app.settings.get_current_theme().clone();
    if app.theme.name != current_theme.name {
        app.theme = current_theme.clone();
        ctx.set_visuals(app.theme.apply_to_visuals());
        
        // Update bot and human text colors
        app.bot_text_color = app.theme.bot_text_color;
        app.user_text_color = app.theme.user_text_color;
        
        // Force a redraw of the chat messages
        app.chat.clear_syntax_cache();
        app.chat.set_has_updates();
    }

    if ctx.input(|i| i.key_pressed(Key::Minus) && i.modifiers.ctrl) {
        change_ui_scale(app, ctx, false);
    }

    if ctx.input(|i| i.key_pressed(Key::Plus) && i.modifiers.ctrl) {
        change_ui_scale(app, ctx, true);
    }

    let current_pixels_per_point = ctx.pixels_per_point();
    if (current_pixels_per_point - app.settings.ui_scale).abs() > 0.001 {
        app.settings.ui_scale = current_pixels_per_point;
        ctx.set_pixels_per_point(app.settings.ui_scale);
    }

    if app.chat.has_updates() {
        app.state.update(&mut app.chat);
    }

    crate::app::app_ui::render(app, ctx);
}

pub fn switch_provider(app: &mut ChatbotApp, model: String) {
    let (provider, is_custom) = if model == "Other" {
        (app.providers.iter().find(|p| p.name() == app.ui.selected_provider), true)
    } else if model.starts_with("accounts/fireworks/models/") {
        (app.providers.iter().find(|p| p.name() == "Fireworks"), true)
    } else {
        (app.providers.iter().find(|p| p.models().contains(&model.as_str())), false)
    };

    if let Some(current_provider) = provider {
        println!("Switching to provider: {} with model: {}", current_provider.name(), model);

        app.chat.update_provider(Arc::clone(current_provider));

        let model_to_use = if is_custom {
            app.ui.custom_model_name.clone()
        } else {
            model.clone()
        };

        let providers_clone = app.providers.clone();
        if let Some(chatbot) = Arc::get_mut(&mut app.chat.chatbot) {
            chatbot.switch_model(&providers_clone, model_to_use.clone());
        }

        if let Ok(mut current_model) = app.chat.current_model.lock() {
            *current_model = model_to_use;
        }

        println!("Provider and model updated successfully");
    } else {
        println!("Error: No provider found for model: {}", model);
    }
}

pub fn reload_providers(app: &mut ChatbotApp) {
    let api_keys = app.settings.get_api_keys();
    app.providers = ChatbotApp::create_providers(&api_keys);

    app.ui.selected_provider = app.providers[0].name().to_string();
    app.ui.selected_model = app.providers[0].models()[0].to_string();
    app.chat.update_provider(Arc::clone(&app.providers[0]));
}

pub fn change_ui_scale(app: &mut ChatbotApp, ctx: &egui::Context, increase: bool) {
    let now = Instant::now();
    if now.duration_since(app.last_scale_change) > std::time::Duration::from_millis(200) {
        if increase {
            app.settings.ui_scale = (app.settings.ui_scale + 0.1).min(2.0);
        } else {
            app.settings.ui_scale = (app.settings.ui_scale - 0.1).max(0.5);
        }
        ctx.set_pixels_per_point(app.settings.ui_scale);
        app.last_scale_change = now;
    }
}