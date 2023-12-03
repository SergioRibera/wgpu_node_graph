use eframe::egui::{self, Ui};

const DEFAULT_SIZE: f32 = 500.;

#[derive(Default)]
pub struct Search {
    pub input: String,
}

impl Search {
    pub fn show_window(&mut self, ctx: &egui::Context, ui: &mut Ui) {
        let x = (ui.available_width() / 2.) - DEFAULT_SIZE;
        let y = ui.available_height() - 50.;

        egui::Window::new("Search")
            .auto_sized()
            .movable(true)
            .resizable(false)
            .title_bar(false)
            .collapsible(false)
            .default_open(true)
            .min_size((DEFAULT_SIZE, 50.))
            .default_pos((x, y))
            .show(ctx, |ui| {
                let placeholder_id = ui.label("Search: ").id;
                ui.text_edit_singleline(&mut self.input).labelled_by(placeholder_id)
            });
    }
}
