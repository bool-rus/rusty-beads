use crate::model::{Color, ColorTrait};
use crate::model::beads::BeadsLine;
use egui::Color32;
use serde::Deserialize;


fn load_from_string<'de, T: ColorTrait + Deserialize<'de>>(s: &'de str) -> Result<BeadsLine<T>, String> {
    serde_json::from_str(s).map_err(|e|e.to_string())
}

fn load_compat(s: &str) -> Result<BeadsLine<Color32>, String> {
    match load_from_string(&s) {
        Ok(result) => Ok(result),
        Err(_) => Ok(load_from_string::<Color>(&s)?.map(|c|c.clone().into())),
    }
}

#[cfg(not(target_arch="wasm32"))]
pub use native::{open_file, save_file};

#[cfg(target_arch="wasm32")]
pub use wasm::*;

#[cfg(not(target_arch="wasm32"))]
mod native {
    use super::*;
    use std::fs::File;
    use std::io::{Write, Read};
    
    use std::path::PathBuf;

    fn load_to_string(path: &PathBuf) -> Result<String, String> {
        let mut file = File::open(path).map_err(|e| e.to_string())?;
        let mut buf = String::new();
        file.read_to_string(&mut buf).map_err(|e|e.to_string())?;
        Ok(buf)
    }

    fn load_line(path: &PathBuf) -> Result<BeadsLine<egui::Color32>, String> {
        let s = load_to_string(path)?;
        load_compat(&s)
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

    fn save(path: &PathBuf, line: &BeadsLine<egui::Color32>) -> Result<(), String> {
        let mut file = File::create(path)
            .map_err(|e|e.to_string())?;
        let serialized = serde_json::to_string(line)
            .map_err(|e|e.to_string())?;
        file.write_all(serialized.as_bytes())
            .map_err(|e|e.to_string())
    }
} 


#[cfg(target_arch="wasm32")]
mod wasm {
    use super::*;
    use wasm_bindgen::prelude::*;
    use serde::Deserialize;

    #[wasm_bindgen]
    extern "C" {
        fn open_file_dialog();
        fn get_file_content() -> Option<String>;
        fn send_file(data: &str);
    }
    pub fn open_file() {
        open_file_dialog();
    }

    pub fn save_file(line: &BeadsLine<Color32>) -> Result<(), String> {
        let s = serde_json::to_string(line).unwrap();
        send_file(&s);
        Ok(())
    }

    pub fn invoke_beads() -> Option<Result<BeadsLine<Color32>, String>>{
        let text = get_file_content()?;
        Some(load_compat(&text))
    }
}