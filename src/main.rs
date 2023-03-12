#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] 
// hide console window on Windows in release
use eframe::{egui, CreationContext};
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


#[cfg(not(target_arch="wasm32"))]
fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Rusty Beads",
        options,
        Box::new(|_cc| Box::new(app::BeadApp::default())),
    );
}

#[cfg(target_arch = "wasm32")]
pub fn main() {
    let web_options = eframe::WebOptions::default();
    eframe::start_web(
        "rusty-beads",
        web_options,
        Box::new(|_cc| Box::new(app::BeadApp::default())),
    ).unwrap();
}

pub fn rich(text: &str) -> RichText {
    RichText::new(text)
}
