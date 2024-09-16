use crate::app::ChatbotApp;
use eframe::egui::{self, SidePanel, CentralPanel, Layout, Align};

pub fn render(app: &mut ChatbotApp, ctx: &egui::Context) {
    SidePanel::left("chat_history_panel")
        .resizable(true)
        .default_width(200.0)
        .width_range(150.0..=400.0)
        .show(ctx, |ui| {
            ui.with_layout(Layout::top_down_justified(Align::LEFT), |ui| {
                let available_height = ui.available_height();
                let bottom_panel_height = 100.0;

                egui::ScrollArea::vertical().max_height(available_height - bottom_panel_height).show(ui, |ui| {
                    app.state.render_chat_history(ui, &mut app.chat, &app.icons, &app.theme);
                });

                ui.with_layout(Layout::bottom_up(Align::LEFT), |ui| {
                    ui.set_min_height(bottom_panel_height);
                    app.state.render_bottom_left_section(ui, &mut app.chat, &mut app.settings, &mut app.ui, &app.providers, &app.theme);
                });
            });
        });

    CentralPanel::default().show(ctx, |ui| {
        app.ui.render(ui, &mut app.chat, &mut app.settings, &app.icons, &app.providers, &app.theme , &mut app.current_profile, &mut app.state);

        if let Some(previous_model) = app.state.previous_model.take() {
            if previous_model != app.ui.selected_model {
                app.switch_provider(app.ui.selected_model.clone());
            }
        }
        app.state.previous_model = Some(app.ui.selected_model.clone());
    });
}