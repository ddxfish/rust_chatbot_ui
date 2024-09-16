use super::Theme;
use egui::Color32;

pub fn get_light_theme() -> Theme {
    Theme {
        name: "Light".to_string(),
        panel_fill: Color32::from_gray(240),
        window_fill: Color32::from_gray(230),
        extreme_bg_color: Color32::from_gray(220),
        override_text_color: Color32::from_rgb(20, 20, 20),
        noninteractive_bg_fill: Color32::from_gray(230),
        inactive_bg_fill: Color32::from_gray(220),
        user_message_bg: Color32::from_rgb(220, 220, 220),
        user_message_border: Color32::from_rgb(0, 100, 200),
        bot_message_bg: Color32::from_rgb(240, 240, 240),
        bot_message_border: Color32::from_rgb(100, 0, 100),
        selected_chat_color: Color32::BLUE,
        unselected_chat_color: Color32::BLACK,
        input_text_color: Color32::from_rgb(20, 20, 20),
        settings_text_color: Color32::from_rgb(40, 40, 40),
        settings_button_text_color: Color32::from_rgb(20, 20, 20),
        settings_button_bg_color: Color32::from_rgb(200, 200, 200),
        settings_title_color: Color32::from_rgb(255, 255, 255),
        dropdown_text_color: Color32::from_rgb(20, 20, 20),
        new_chat_button_text_color: Color32::from_rgb(20, 20, 20),
        trash_button_bg_color: Color32::from_rgba_premultiplied(255, 0, 0, 100),
        button_text_color: Color32::from_rgb(20, 20, 20),
        button_bg_color: Color32::from_rgb(200, 200, 200),
        model_provider_dropdown_text_color: Color32::from_rgb(20, 20, 20),
        model_provider_dropdown_bg_color: Color32::from_rgb(230, 230, 230),
        theme_dropdown_bg_color: Color32::from_rgb(230, 230, 230),
        bot_name_text_color: Color32::from_rgb(100, 0, 100),
        user_name_text_color: Color32::from_rgb(0, 100, 200),
        bot_text_color: Color32::from_rgb(100, 0, 100),
        user_text_color: Color32::from_rgb(0, 100, 200),
        code_block_bg: Color32::from_rgb(128, 128, 128),  // Changed to a darker shade
        code_block_border: Color32::from_rgb(200, 200, 200),  // Slightly darker border
        code_block_language_color: Color32::from_rgb(100, 100, 100),  // Darker text for better contrast
    }
}