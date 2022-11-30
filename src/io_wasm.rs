use super::*;
use wasm_bindgen::prelude::*;
use serde::Deserialize;

#[wasm_bindgen]
extern "C" {
    fn open_file_dialog();
    fn get_file_content() -> Option<String>;
}
pub fn open_file() {
    open_file_dialog();
}

pub fn save_file(line: &BeadsLine<Color32>) -> Result<(), String> {
    Err("unimplemented!".to_string())
}

pub fn invoke_beads() -> Option<Result<BeadsLine<Color32>, String>>{
    let text = get_file_content()?;
    Some(serde_json::from_str(&text).map_err(|e|e.to_string()))
}