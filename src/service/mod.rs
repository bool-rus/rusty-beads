mod grid;
pub use grid::{Service as GridService, Message as GridServiceMessage};

pub trait Service {
    type Message;
    fn service(&mut self, msg: Self::Message) -> Option<Self::Message>;
}

