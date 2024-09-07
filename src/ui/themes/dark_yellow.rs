use super::Theme;
use egui::Color32;

pub fn get_dark_yellow_theme() -> Theme {
    Theme {
        name: "Dark Yellow".to_string(),
        panel_fill: Color32::from_rgb(64, 64, 0),
        window_fill: Color32::from_rgb(80, 80, 0),
        extreme_bg_color: Color32::from_rgb(48, 48, 0),
        override_text_color: Color32::from_rgb(255, 255, 200),
        noninteractive_bg_fill: Color32::from_rgb(96, 96, 0),
        inactive_bg_fill: Color32::from_rgb(112, 112, 0),
        user_message_bg: Color32::from_rgb(128, 128, 0),
        user_message_border: Color32::from_rgb(255, 255, 0),
        bot_message_bg: Color32::from_rgb(96, 96, 0),
        bot_message_border: Color32::from_rgb(204, 204, 0),
        selected_chat_color: Color32::from_rgb(255, 255, 0),
        unselected_chat_color: Color32::from_rgb(204, 204, 0),
        input_text_color: Color32::from_rgb(255, 255, 200),
        settings_text_color: Color32::from_rgb(255, 255, 200),
        settings_button_text_color: Color32::from_rgb(64, 64, 0),
        settings_button_bg_color: Color32::from_rgb(255, 255, 0),
        settings_title_color: Color32::from_rgb(255, 255, 255),
        dropdown_text_color: Color32::from_rgb(255, 255, 200),
        new_chat_button_text_color: Color32::from_rgb(64, 64, 0),
        trash_button_bg_color: Color32::from_rgba_premultiplied(200, 0, 0, 100),
        button_text_color: Color32::from_rgb(64, 64, 0),
        button_bg_color: Color32::from_rgb(255, 255, 0),
        model_provider_dropdown_text_color: Color32::from_rgb(255, 255, 200),
        model_provider_dropdown_bg_color: Color32::from_rgb(112, 112, 0),
        theme_dropdown_bg_color: Color32::from_rgb(112, 112, 0),
        bot_name_text_color: Color32::from_rgb(255, 255, 0),
        user_name_text_color: Color32::from_rgb(255, 255, 200),
        bot_text_color: Color32::from_rgb(204, 204, 0),
        user_text_color: Color32::from_rgb(255, 255, 0),
    }
}