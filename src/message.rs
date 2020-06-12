use crate::entities::StandartMessage;
use crate::ui::{GridMessage, RightMenuMessage};

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Standart(StandartMessage),
    Grid(GridMessage),
    RightMenu(RightMenuMessage),
}

impl From<RightMenuMessage> for Message {
    fn from(m: RightMenuMessage) -> Self {
        Message::RightMenu(m)
    }
}

impl From<StandartMessage> for Message {
    fn from(m: StandartMessage) -> Self {
        Message::Standart(m)
    }
}

impl From<GridMessage> for Message {
    fn from(m: GridMessage) -> Self {
        Message::Grid(m)
    }
}