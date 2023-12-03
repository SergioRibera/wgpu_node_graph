use eframe::egui::{self, Ui};

const DEFAULT_SIZE: f32 = 300.;

pub fn show_window(ctx: &egui::Context, ui: &mut Ui) {
    let x = ui.available_width() - DEFAULT_SIZE - 50.;
    let y = ui.available_height() - DEFAULT_SIZE - 50.;

    egui::Window::new("Preview")
        .auto_sized()
        .movable(true)
        .resizable(true)
        .title_bar(true)
        .collapsible(true)
        .default_open(true)
        .min_size((DEFAULT_SIZE, DEFAULT_SIZE))
        .default_pos((x, y))
        .show(ctx, |_ui| {});
}
