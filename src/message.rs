use crate::ui::{GridMessage, RightMenuMessage, TopMenuMessage, PaletteMessage};

#[derive(Debug, Clone, Copy)]
pub enum Message {
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