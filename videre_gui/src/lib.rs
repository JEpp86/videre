mod apps;
mod videre_gui;

pub use apps::{Pane, View};
pub use videre_gui::VidGUI;

/*
use std::collections::BTreeSet;

struct Panes {
    panes: Vec<Box<dyn Pane>>,
    open: BTreeSet<String>,
}

impl Default for Panes {
    fn default() -> Self {
        Self::from_panes(vec![Box::<super::image_viewer::ImageViewer>::default()])
    }
}

impl Panes {
    pub fn from_panes(panes: Vec<Box<dyn Pane>>) -> Self {
        let open: BTreeSet<String> = BTreeSet::new();

        Self { panes, open }
    }
}
*/
