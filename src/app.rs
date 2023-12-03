use eframe::{egui, glow};
use eframe::egui::Modifiers;

use crate::nodes_context_menu::NodeContextMenu;
use crate::preview::PreviewWindow;

pub struct Application<'a> {
    show_settings: bool,
    preview: PreviewWindow,
    node_ctx_menu: NodeContextMenu<'a>,
}

impl eframe::App for Application<'_> {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                self.file_menu_button(ui);
            });
        });

        if self.show_settings {
            egui::SidePanel::left("wgpu_node_graph_settings")
                .resizable(true)
                .default_width(250.)
                .max_width(500.)
                .show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.heading("Settings");
                    });

                    ui.separator();
                });
        }

        egui::CentralPanel::default()
            .show(ctx, |ui| {
                self.preview.show(ctx, ui);
            })
            .response
            .context_menu(|ui| self.node_ctx_menu.render(ui));
    }
}

impl Application<'_> {
    pub fn new(gl: &glow::Context) -> Self {
        Self {
            show_settings: true,
            preview: PreviewWindow::new(gl, eframe::Renderer::Glow),
            node_ctx_menu: Default::default(),
        }
    }
    fn file_menu_button(&mut self, ui: &mut egui::Ui) {
        let save_shortcut = egui::KeyboardShortcut::new(Modifiers::CTRL, egui::Key::S);
        let open_shortcut = egui::KeyboardShortcut::new(Modifiers::CTRL, egui::Key::O);

        if ui.add(egui::Button::new("Toggle Settings")).clicked() {
            self.show_settings = !self.show_settings;
        }

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
