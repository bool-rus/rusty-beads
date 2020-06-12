use crate::entities::StandartMessage;
use crate::ui::GridMessage;

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Standart(StandartMessage),
    Grid(GridMessage),
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