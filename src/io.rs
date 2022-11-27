use crate::model::{Color, ColorTrait};
use std::fs::File;
use std::io::{Write, BufReader};
use std::path::PathBuf;
use crate::model::beads::BeadsLine;
use egui::Color32;
use serde::Deserialize;

fn save(path: &PathBuf, line: &BeadsLine<egui::Color32>) -> Result<(), String> {
    let mut file = File::create(path)
        .map_err(|e|e.to_string())?;
    let serialized = serde_json::to_string(line)
        .map_err(|e|e.to_string())?;
    file.write_all(serialized.as_bytes())
        .map_err(|e|e.to_string())
}

fn load<'de, T: ColorTrait + Deserialize<'de>>(path: &PathBuf) -> Result<BeadsLine<T>, String> {
    let file = File::open(path).map_err(|e| e.to_string())?;
    let reader = BufReader::new(file);
    let mut deserializer = serde_json::Deserializer::from_reader(reader);
    BeadsLine::deserialize(&mut deserializer).map_err(|e|e.to_string())
}

fn load_line(path: &PathBuf) -> Result<BeadsLine<egui::Color32>, String> {
    match load(path) {
        Ok(result) => Ok(result),
        Err(_) => Ok(load::<Color>(path)?.map(|c|c.clone().into())),
    }
}

pub fn open_file() -> Result<BeadsLine<Color32>, String> {
    if let Some(path) = rfd::FileDialog::new().pick_file() {
        load_line(&path)
    } else {
        Err("file not picked".to_string())
    }
}

pub fn save_file(line: &BeadsLine<Color32>) -> Result<(), String> {
    if let Some(path) = rfd::FileDialog::new().save_file() {
        save(&path, &line)
    } else {
        Err("path not selected".to_string())
    }
}