use crate::entities::StandartMessage;
use crate::ui::{GridMessage, RightMenuMessage, TopMenuMessage, PaletteMessage};

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Standart(StandartMessage),
    Grid(GridMessage),
    RightMenu(RightMenuMessage),
    TopMenu(TopMenuMessage),
}

impl From<TopMenuMessage> for Message {
    fn from(m: TopMenuMessage) -> Self {
        Message::TopMenu(m)
    }
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

impl From<PaletteMessage> for Message {
    fn from(m: PaletteMessage) -> Self {
        Message::TopMenu(TopMenuMessage::Pallette(m))
    }
}