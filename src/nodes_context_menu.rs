use eframe::egui;

pub trait ContextMenuItem {
    fn render(&self, _search: &str, ui: &mut egui::Ui);
}

pub fn can_show(search: &str, names: &[&str]) -> bool {
    search.is_empty()
        || names.iter().any(|n| {
            search
                .split_whitespace()
                .any(|s| n.to_ascii_lowercase().contains(&s.to_ascii_lowercase()))
        })
}

pub struct NodeContextMenu<'a> {
    search: String,
    options: Vec<Box<&'a dyn ContextMenuItem>>,
}

impl Default for NodeContextMenu<'_> {
    fn default() -> Self {
        Self {
            search: String::new(),
            options: vec![Box::new(&GeneralItem)],
        }
    }
}

impl<'a> NodeContextMenu<'a> {
    pub fn render(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            let placeholder_id = ui.label("Search: ").id;
            ui.text_edit_singleline(&mut self.search)
                .labelled_by(placeholder_id)
                .request_focus();
        });
        self.options
            .iter()
            .for_each(|opts| opts.render(&self.search, ui));
    }
}

struct GeneralItem;

impl ContextMenuItem for GeneralItem {
    fn render(&self, search: &str, ui: &mut egui::Ui) {
        if can_show(search, &["math", "add", "sum"]) {
            ui.menu_button("Math", |ui| {
                let _ = ui.button("Add");
                let _ = ui.button("Sub");
                let _ = ui.button("Mul");
            });
        }
        if can_show(search, &["texture", "add", "sum"]) {
            ui.menu_button("Texture", |ui| {
                let _ = ui.button("Add");
                let _ = ui.button("Sub");
                let _ = ui.button("Mul");
            });
        }
        if can_show(search, &["Color", "add", "sum"]) {
            ui.menu_button("Color", |ui| {
                let _ = ui.button("Add");
                let _ = ui.button("Sub");
                let _ = ui.button("Mul");
            });
        }
    }
}
