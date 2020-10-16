use crate::grid::Grid;
use crate::entities::Color;
use std::sync::Arc;
use std::path::PathBuf;
use crate::model::Model;

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
            Open(path) => Some(Loaded(
                Arc::new(
                    crate::io::read(path)
                        .map_err(|e|e.to_string())?
                        .map(|c|c.into())
                        .into()
                )
            )),
            Save(path) => {
                crate::io::write(path, self.model.grid_color().as_table()).map_err(|e|e.to_string())?;
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