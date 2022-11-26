use crate::model::{Grid, ColorBead, Color, Bead};
use std::fs::File;
use std::io::{Write, BufReader};
use std::str::FromStr;
use std::num::NonZeroUsize;
use std::path::{Path, PathBuf};
use crate::model::beads::BeadsLine;
use serde::Deserialize;

pub fn save(path: &PathBuf, line: &BeadsLine<egui::Color32>) -> Result<(), String> {
    let mut file = File::create(path)
        .map_err(|e|e.to_string())?;
    let serialized = serde_json::to_string(line)
        .map_err(|e|e.to_string())?;
    file.write_all(serialized.as_bytes())
        .map_err(|e|e.to_string())
}

pub fn load_line(path: &PathBuf) -> Result<BeadsLine<egui::Color32>, String> {
    let file = File::open(path).map_err(|e| e.to_string())?;
    let reader = BufReader::new(file);
    let mut deserializer = serde_json::Deserializer::from_reader(reader);
    BeadsLine::deserialize(&mut deserializer).map_err(|e|e.to_string())
}

