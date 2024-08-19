use egui::{Ui, TextEdit, Button, Vec2, Image};
use crate::chat::Chat;
use crate::app::Icons;
use crate::ui::theme::DarkTheme;

pub fn render_input(ui: &mut Ui, chat: &mut Chat, icons: &Icons, input: &mut String, is_loading: &mut bool, theme: &DarkTheme) {
    let input_field = TextEdit::multiline(input)
        .desired_rows(3)
        .hint_text("Type your message here...")
        .font(egui::FontId::proportional(16.0))
        .text_color(theme.override_text_color);

    let response = ui.add_sized(
        [ui.available_width(), 50.0],
        input_field
    );

    let button_size = Vec2::new(20.0, 20.0);
    let button_pos = ui.min_rect().right_bottom() - button_size - Vec2::new(12.0, 38.0);
    
    if ui.put(egui::Rect::from_min_size(button_pos, button_size), Button::image(Image::new(&icons.send).fit_to_exact_size(button_size))).clicked()
       || (ui.input(|i| i.key_pressed(egui::Key::Enter) && !i.modifiers.shift)) {
        if !input.trim().is_empty() {
            chat.process_input(std::mem::take(input));
            *is_loading = true;
        }
        response.request_focus();
    }
}