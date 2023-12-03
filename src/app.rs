use eframe::egui;
use eframe::egui::Modifiers;

use crate::preview;

#[derive(Default)]
pub struct Application;

impl eframe::App for Application {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                self.file_menu_button(ui);
            });
        });
        egui::SidePanel::left("wgpu_node_graph_settings")
            .resizable(true)
            .default_width(250.0)
            .max_width(500.)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("Settings");
                });

                ui.separator();
            });
        egui::CentralPanel::default().show(ctx, |ui| {
            preview::show_window(ctx, ui);
        });
    }
}

impl Application {
    fn file_menu_button(&mut self, ui: &mut egui::Ui) {
        let save_shortcut = egui::KeyboardShortcut::new(Modifiers::CTRL, egui::Key::S);
        let open_shortcut = egui::KeyboardShortcut::new(Modifiers::CTRL, egui::Key::O);

        ui.menu_button("File", |ui| {
            ui.set_min_width(220.0);
            ui.style_mut().wrap = Some(false);
            if ui
                .add(
                    egui::Button::new("Save")
                        .shortcut_text(ui.ctx().format_shortcut(&save_shortcut)),
                )
                .clicked()
            {
                ui.close_menu();
            }

            if ui
                .add(
                    egui::Button::new("Open")
                        .shortcut_text(ui.ctx().format_shortcut(&open_shortcut)),
                )
                .clicked()
            {
                ui.close_menu();
            }
        });
    }
}
