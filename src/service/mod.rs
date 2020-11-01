use crate::model::*;

mod io;
mod message;
mod grid;

use grid::{Service as GridService, Message as GridServiceMessage};
use io::{Service as IOService, Message as IOMessage};
use crate::message::Message;


pub trait Service {
    type Message;
    fn service(&mut self, msg: Self::Message) -> Result<Option<Self::Message>, String>;
}

pub struct AppService {
    grid: GridService<Color>,
    io: IOService,
}

impl AppService {
    pub fn new(model: Model<Color>) -> Self {
        Self {
            grid: GridService::new(model),
            io: Default::default(),
        }
    }
    fn process_with_result(&mut self, msg: Message) -> Result<Option<Message>, String> {
        let grid_msg;
        if let Some(io_response) = self.io.service(msg.clone().into())? {
            grid_msg = io_response.into();
        } else {
            grid_msg = msg.into();
        }
        let grid_response = self.grid.service(grid_msg)?;
        if let Some(msg) = grid_response.clone() {
            self.io.service(msg.into())?;
        }
        Ok(grid_response.map(From::from))
    }

    pub fn process(&mut self, msg: Message) -> Option<Message> {
        self.process_with_result(msg).unwrap_or_else(|e|Some(Message::Error(e)))
    }
}
