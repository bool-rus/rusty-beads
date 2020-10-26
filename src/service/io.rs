use std::sync::Arc;
use std::path::PathBuf;
use crate::model::*;

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
                let model = Arc::new(match crate::io::load_line(&path) {
                    Ok(line) => {
                        Model::from(line)
                    }
                    Err(e) => {
                        let grid = crate::io::load_grid(&path)?;
                        Model::from(grid)
                    }
                });
                self.model = model.clone();
                Some(Loaded(model))
            },
            Save(path) => {
                crate::io::save(&path, self.model.line())?;
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