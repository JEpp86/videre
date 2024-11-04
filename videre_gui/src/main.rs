use eframe::egui;
use videre_gui as vg;

fn main() -> eframe::Result {
    // env_logger::init();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Videre",
        options,
        Box::new(|_cc| Ok(Box::<vg::VidGUI>::default())),
    )
}
