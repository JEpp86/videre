use crate::apps;
use eframe::{egui, App};
use rfd;

pub struct State {
    app_panes: apps::Panes,
    workspace: Option<String>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            workspace: None,
            app_panes: apps::Panes::default(),
        }
    }
}

pub struct VidGUI {
    state: State,
    file_path: Option<String>,
}

impl Default for VidGUI {
    fn default() -> Self {
        Self {
            state: State::default(),
            file_path: None,
        }
    }
}

impl VidGUI {
    fn file_menu(&mut self, ui: &mut egui::Ui) {
        ui.menu_button("File", |ui| {
            ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);

            if ui.add(egui::Button::new("Open")).clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_file() {
                    self.file_path = Some(path.display().to_string());
                }
            }
        });
    }
}

impl App for VidGUI {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                self.file_menu(ui);
            })
        });
        // self.state.app_panes.ui(ctx);
        /* simple pane, but w're going to abstract this
        egui::CentralPanel::default().show(ctx, |ui| {
            // create a button to select file path
            if ui.button("Open file...").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_file() {
                    self.file_path = Some(path.display().to_string());
                }
            }

            // display file path and image
            if let Some(file_path) = &self.file_path {
                ui.horizontal(|ui| {
                    ui.label("File path:");
                    ui.monospace(file_path);
                });
                ui.add(egui::Image::new(format!("file://{file_path}")));
            }
        });
        */
    }
}
