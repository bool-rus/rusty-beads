
use eframe::wasm_bindgen::{self, prelude::*};

use eframe::egui;
use egui::*;
use model::*;
use beads::BeadsRow;
use settings::Settings;

mod wrapper;
mod model;
mod palette;
mod settings;
mod summary;
mod io;
mod app;


#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub async fn start()  {
    let web_options = eframe::WebOptions::default();
    eframe::start_web(
        "rusty-beads",
        web_options,
        Box::new(|_cc| Box::new(app::BeadApp::default())),
    ).await.unwrap();
}

pub fn rich(text: &str) -> RichText {
    RichText::new(text)
}