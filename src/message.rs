use crate::ui::{GridMessage, RightMenuMessage, TopMenuMessage, PaletteMessage, RightPanelMessage};

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Grid(GridMessage),
    RightMenu(RightMenuMessage),
    TopMenu(TopMenuMessage),
    RightPanel(RightPanelMessage),
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

impl From<RightPanelMessage> for Message {
    fn from(m: RightPanelMessage) -> Self {
        Message::RightPanel(m)
    }
}