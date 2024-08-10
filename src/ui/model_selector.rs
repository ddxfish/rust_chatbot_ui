use egui::{ComboBox, Ui, Window, TextEdit};

pub struct ModelSelector {
    selected_model: String,
    custom_model_input: String,
    show_custom_model_popup: bool,
}

impl ModelSelector {
    pub fn new() -> Self {
        Self {
            selected_model: String::new(),
            custom_model_input: String::new(),
            show_custom_model_popup: false,
        }
    }

    pub fn render(&mut self, ui: &mut Ui, models: &[&str]) {
        ComboBox::from_label("Model")
            .selected_text(&self.selected_model)
            .show_ui(ui, |ui| {
                for &model in models {
                    ui.selectable_value(&mut self.selected_model, model.to_string(), model);
                }
                if ui.selectable_label(false, "Other").clicked() {
                    self.show_custom_model_popup = true;
                }
            });

        if self.show_custom_model_popup {
            Window::new("Custom Model")
                .collapsible(false)
                .resizable(false)
                .show(ui.ctx(), |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Enter custom model name:");
                        ui.text_edit_singleline(&mut self.custom_model_input);
                    });
                    ui.horizontal(|ui| {
                        if ui.button("Cancel").clicked() {
                            self.show_custom_model_popup = false;
                        }
                        if ui.button("OK").clicked() {
                            self.selected_model = self.custom_model_input.clone();
                            self.show_custom_model_popup = false;
                        }
                    });
                });
        }
    }

    pub fn selected_model(&self) -> &str {
        &self.selected_model
    }
}