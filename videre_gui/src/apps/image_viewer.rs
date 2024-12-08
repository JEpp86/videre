use crate::apps::{Pane, View};
use eframe::egui;
pub struct ImageViewer {
    path: Option<String>,
}

impl Default for ImageViewer {
    fn default() -> Self {
        Self { path: None }
    }
}

impl Pane for ImageViewer {
    fn name(&self) -> &'static str {
        "Image Viewer"
    }

    fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        egui::Window::new(self.name())
            .default_width(320.0)
            .default_height(480.0)
            .open(open)
            .resizable([true, false])
            .show(ctx, |ui| {
                use View as _;
                self.ui(ui);
            });
    }
}

impl View for ImageViewer {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.heading("Image");
    }
}
