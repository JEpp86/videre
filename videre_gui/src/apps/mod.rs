use eframe::egui::{self, Context, Ui};
use std::collections::BTreeSet;
pub mod image_viewer;

pub trait View {
    fn ui(&mut self, ui: &mut egui::Ui);
}

pub trait Pane {
    fn enabled(&self, _ctx: &egui::Context) -> bool {
        true
    }

    fn name(&self) -> &'static str;

    fn show(&mut self, ctx: &egui::Context, open: &mut bool);
}

pub struct Panes {
    panes: Vec<Box<dyn Pane>>,
    open: BTreeSet<String>,
}

impl Default for Panes {
    fn default() -> Self {
        Self::from_panes(vec![
            Box::<super::apps::image_viewer::ImageViewer>::default(),
        ])
    }
}

impl Panes {
    pub fn from_panes(panes: Vec<Box<dyn Pane>>) -> Self {
        let open: BTreeSet<String> = BTreeSet::new();

        Self { panes, open }
    }
    /*/
    pub fn ui(&mut self, ctx: &Context) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| egui::menu::bar(ui, |ui| {}))
    }
    */
}
