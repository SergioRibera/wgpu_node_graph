use eframe::egui::Modifiers;
use eframe::{egui, CreationContext, Renderer};
use egui_node_graph::GraphEditorState;

use crate::nodes::{ShaderEditorState, ShaderNodeGraphState, ShaderNodeTemplates};
use crate::preview::PreviewWindow;

const PERSISTENCE_KEY: &'static str = "SergioRibera_ShaderNodeGraph";

pub struct Application {
    node_editor_state: ShaderEditorState,
    show_settings: bool,
    preview: PreviewWindow,
    nodes_state: ShaderNodeGraphState,
}

impl eframe::App for Application {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, PERSISTENCE_KEY, &self.nodes_state);
    }
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
                let _ = self.node_editor_state.draw_graph_editor(
                    ui,
                    ShaderNodeTemplates,
                    &mut self.nodes_state,
                    Vec::new()
                );
                self.preview.show(ctx, ui);
            });
    }
}

impl Application {
    pub fn new(cc: &CreationContext, renderer: Renderer) -> Self {
        let gl = if renderer == Renderer::Glow {
            cc.gl.as_ref()
        } else {
            None
        };

        Self {
            show_settings: true,
            preview: PreviewWindow::new(gl.map(|r| r.as_ref()), renderer),
            nodes_state: Default::default(),
            node_editor_state: GraphEditorState::default(),
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
