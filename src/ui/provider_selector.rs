use egui::{ComboBox, Ui};
use crate::providers::{Provider, get_providers};

pub fn render(ui: &mut Ui, selected_provider: &mut Box<dyn Provider>) {
    ComboBox::from_label("Provider")
        .selected_text(selected_provider.name())
        .show_ui(ui, |ui| {
            for provider in get_providers() {
                ui.selectable_value(selected_provider, provider, provider.name());
            }
        });
}
