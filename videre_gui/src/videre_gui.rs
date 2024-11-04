use eframe::egui;

pub struct VidGUI {
    file_path: String,
}

impl Default for VidGUI {
    fn default() -> Self {
        Self {
            file_path: "~".to_owned(),
        }
    }
}

impl eframe::App for VidGUI {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Videre");
            ui.horizontal(|ui| {
                let path_name = ui.label("File path: ");
                ui.text_edit_singleline(&mut self.file_path)
                    .labelled_by(path_name.id);
            });
        });
    }
}
