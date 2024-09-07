use super::Theme;
use egui::Color32;

pub fn get_olive_and_tan_theme() -> Theme {
    Theme {
        name: "Olive and Tan".to_string(),
        panel_fill: Color32::from_rgb(67, 84, 58),
        window_fill: Color32::from_rgb(240, 234, 214),
        extreme_bg_color: Color32::from_rgb(155, 170, 148),
        override_text_color: Color32::from_rgb(50, 50, 50),
        noninteractive_bg_fill: Color32::from_rgb(180, 190, 165),
        inactive_bg_fill: Color32::from_rgb(120, 140, 100),
        user_message_bg: Color32::from_rgb(245, 235, 215),
        user_message_border: Color32::from_rgb(220, 210, 200),
        bot_message_bg: Color32::from_rgb(230, 238, 220),
        bot_message_border: Color32::from_rgb(150, 170, 130),
        selected_chat_color: Color32::from_rgb(220, 230, 200),
        unselected_chat_color: Color32::from_rgb(120, 140, 90),
        input_text_color: Color32::from_rgb(40, 40, 0),
        settings_text_color: Color32::from_rgb(50, 50, 50),
        settings_button_text_color: Color32::from_rgb(240, 234, 214),
        settings_button_bg_color: Color32::from_rgb(100, 120, 80),
        settings_title_color: Color32::from_rgb(255, 255, 255),
        dropdown_text_color: Color32::from_rgb(50, 50, 50),
        new_chat_button_text_color: Color32::from_rgb(240, 234, 214),
        trash_button_bg_color: Color32::from_rgba_premultiplied(200, 100, 100, 100),
        button_text_color: Color32::from_rgb(240, 234, 214),
        button_bg_color: Color32::from_rgb(100, 120, 80),
        model_provider_dropdown_text_color: Color32::from_rgb(50, 50, 50),
        model_provider_dropdown_bg_color: Color32::from_rgb(220, 230, 200),
        theme_dropdown_bg_color: Color32::from_rgb(220, 230, 200),
        bot_name_text_color: Color32::from_rgb(100, 120, 80),
        user_name_text_color: Color32::from_rgb(80, 70, 60),
        bot_text_color: Color32::from_rgb(70, 90, 60),
        user_text_color: Color32::from_rgb(130, 120, 110),
    }
}