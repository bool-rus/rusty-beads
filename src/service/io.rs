use crate::grid::Grid;
use crate::entities::Color;
use std::sync::Arc;
use std::path::PathBuf;
use crate::model::{Model, Bead};
use std::fs::File;
use std::io::{Write, BufReader};
use serde::Deserialize;
use crate::beads::BeadsLine;

#[derive(Debug, Clone)]
pub enum Message {
    Open(PathBuf),
    Save(PathBuf),
    Loaded(Arc<Model<Color>>),
    GridUpdated(Arc<Model<Color>>),
    Ignore,
}
#[derive(Default)]
pub struct Service {
    model: Arc<Model<Color>>
}

impl super::Service for Service {
    type Message = Message;

    fn service(&mut self, msg: Self::Message) -> Result<Option<Self::Message>, String> {
        use Message::*;
        Ok( match msg {
            Open(path) => {
                let file = File::open(path).map_err(|e| e.to_string())?;
                let reader = BufReader::new(file);
                let mut deserializer = serde_json::Deserializer::from_reader(reader);
                let line = BeadsLine::deserialize(&mut deserializer).map_err(|e|e.to_string())?;
                let model = Arc::new(Model::from(line));
                self.model = model.clone();
                Some(Loaded(model))
            },
            Save(path) => {
                let mut file = File::create(path)
                    .map_err(|e|e.to_string())?;
                let serialized = serde_json::to_string(self.model.line())
                    .map_err(|e|e.to_string())?;
                file.write_all(serialized.as_bytes())
                    .map_err(|e|e.to_string())?;
                None
            },
            GridUpdated(model) => {
                self.model = model;
                None
            },
            Ignore | Loaded(_) => None,
        })
    }
}