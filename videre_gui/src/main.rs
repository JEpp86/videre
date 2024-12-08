use eframe::egui;
use egui_extras;
use videre_gui as vg;

fn main() -> eframe::Result {
    // env_logger::init();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([640.0, 512.0])
            .with_drag_and_drop(true),
        ..Default::default()
    };
    eframe::run_native(
        "Videre",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::<vg::VidGUI>::default())
        }),
    )
}
