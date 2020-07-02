use crate::ui::{GridMessage, RightMenuMessage, TopMenuMessage, PaletteMessage, RightPanelMessage, LeftMenuMessage, LeftPanelMessage};

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Grid(GridMessage),
    RightMenu(RightMenuMessage),
    TopMenu(TopMenuMessage),
    LeftPanel(LeftPanelMessage),
    RightPanel(RightPanelMessage),
    LeftMenu(LeftMenuMessage),
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

impl From<LeftMenuMessage> for Message {
    fn from(m: LeftMenuMessage) -> Self {
        Message::LeftMenu(m)
    }
}

impl From<LeftPanelMessage> for Message {
    fn from(m: LeftPanelMessage) -> Self {
        Message::LeftPanel(m)
    }
}

impl From<Message> for TopMenuMessage {
    fn from(msg: Message) -> Self {
        match msg {
            Message::TopMenu(msg) => msg,
            _ => TopMenuMessage::Ignore,
        }
    }
}

impl From<Message> for RightMenuMessage {
    fn from(msg: Message) -> Self {
        match msg {
            Message::RightMenu(msg) => msg,
            _ => RightMenuMessage::Ignore,
        }
    }
}

impl From<Message> for LeftMenuMessage {
    fn from(msg: Message) -> Self {
        match msg {
            Message::LeftMenu(msg) => msg,
            _ => LeftMenuMessage::Ignore,
        }
    }
}

impl From<Message> for GridMessage {
    fn from(msg: Message) -> Self {
        use Message::*;
        match msg  {
            Grid(msg) => msg,
            TopMenu(TopMenuMessage::GridAction(action)) => GridMessage::GridAction(action),
            TopMenu(TopMenuMessage::Undo) => GridMessage::Undo,
            TopMenu(TopMenuMessage::Redo) => GridMessage::Redo,
            LeftMenu(LeftMenuMessage::GridAction(action)) => GridMessage::GridAction(action),
            LeftMenu(LeftMenuMessage::ZoomIn) => GridMessage::ZoomIn,
            LeftMenu(LeftMenuMessage::ZoomOut) => GridMessage::ZoomOut,
            LeftMenu(LeftMenuMessage::SchemaChange) => GridMessage::SchemaChange,
            _ => GridMessage::Ignore
        }
    }
}