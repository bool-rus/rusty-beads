use crate::grid::Grid;
use crate::entities::{Color, Bead};
use std::fs::File;
use std::io::{Write, BufReader};
use quick_csv::Csv;
use std::str::FromStr;
use std::num::NonZeroUsize;
use std::path::{Path, PathBuf};
use crate::beads::BeadsLine;
use serde::Deserialize;

pub fn save(path: &PathBuf, line: &BeadsLine<Bead<Color>>) -> Result<(), String> {
    let mut file = File::create(path)
        .map_err(|e|e.to_string())?;
    let serialized = serde_json::to_string(line)
        .map_err(|e|e.to_string())?;
    file.write_all(serialized.as_bytes())
        .map_err(|e|e.to_string())
}

pub fn load_line(path: &PathBuf) -> Result<BeadsLine<Bead<Color>>, String> {
    let file = File::open(path).map_err(|e| e.to_string())?;
    let reader = BufReader::new(file);
    let mut deserializer = serde_json::Deserializer::from_reader(reader);
    BeadsLine::deserialize(&mut deserializer).map_err(|e|e.to_string())
}


pub fn load_grid<T: AsRef<Path>>(file: T) -> Result<Grid<Bead<Color>>, String> {
    let mut data = Vec::with_capacity(10000usize);
    let csv = Csv::from_file(file).map_err(|e|e.to_string())?;
    let mut first = true;
    let mut width = 0usize;
    for row in csv.into_iter() {
        let row = row.map_err(|e|e.to_string())?;
        if first {
            first = false;
            width = row.len();
        }
        row.columns().map_err(|e|e.to_string())?.for_each(|item| {
            data.push(Color::from_str(item).unwrap())
        })
    }
    let width = NonZeroUsize::new(width).ok_or("invalid width".to_string())?;
    let grid = Grid::frow_raw(width, data)
        .map_err(|e|e.to_string())?
        .map(|item|Bead {
        color: item.clone(),
        filled: false,
    });
    Ok(grid)
}

pub fn default_dir() -> PathBuf {
    dirs::document_dir().unwrap_or(".".into())
}