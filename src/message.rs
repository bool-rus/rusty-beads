use crate::ui::{
    GridMessage as GMsg,
    RightMenuMessage as RMMsg,
    TopMenuMessage as TMMsg,
    PaletteMessage,
    RightPanelMessage as RPMsg,
    LeftMenuMessage as LMMsg,
    LeftPanelMessage as LPMsg,
};

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Grid(GMsg),
    RightMenu(RMMsg),
    TopMenu(TMMsg),
    LeftPanel(LPMsg),
    RightPanel(RPMsg),
    LeftMenu(LMMsg),
}

impl From<TMMsg> for Message {
    fn from(m: TMMsg) -> Self {
        Message::TopMenu(m)
    }
}

impl From<RMMsg> for Message {
    fn from(m: RMMsg) -> Self {
        Message::RightMenu(m)
    }
}

impl From<GMsg> for Message {
    fn from(m: GMsg) -> Self {
        Message::Grid(m)
    }
}

impl From<RPMsg> for Message {
    fn from(m: RPMsg) -> Self {
        Message::RightPanel(m)
    }
}

impl From<LMMsg> for Message {
    fn from(m: LMMsg) -> Self {
        Message::LeftMenu(m)
    }
}

impl From<LPMsg> for Message {
    fn from(m: LPMsg) -> Self {
        Message::LeftPanel(m)
    }
}

impl From<Message> for TMMsg {
    fn from(msg: Message) -> Self {
        match msg {
            Message::TopMenu(msg) => msg,
            Message::LeftMenu(LMMsg::Hide) |
            Message::LeftMenu(LMMsg::ShowResize) => TMMsg::Hide,
            _ => TMMsg::Ignore,
        }
    }
}

impl From<Message> for RMMsg {
    fn from(msg: Message) -> Self {
        match msg {
            Message::RightMenu(msg) => msg,
            _ => RMMsg::Ignore,
        }
    }
}

impl From<Message> for LMMsg {
    fn from(msg: Message) -> Self {
        match msg {
            Message::LeftMenu(msg) => msg,
            Message::TopMenu(TMMsg::Hide) |
            Message::TopMenu(TMMsg::Open) |
            Message::TopMenu(TMMsg::Save) => LMMsg::Hide,
            _ => LMMsg::Ignore,
        }
    }
}

impl From<Message> for GMsg {
    fn from(msg: Message) -> Self {
        use Message::*;
        match msg  {
            Grid(msg) => msg,
            LeftPanel(LPMsg::GridAction(action)) => GMsg::GridAction(action),
            TopMenu(TMMsg::Undo) => GMsg::Undo,
            TopMenu(TMMsg::Redo) => GMsg::Redo,
            LeftMenu(LMMsg::ZoomIn) => GMsg::ZoomIn,
            LeftMenu(LMMsg::ZoomOut) => GMsg::ZoomOut,
            LeftMenu(LMMsg::SchemaChange) => GMsg::SchemaChange,
            _ => GMsg::Ignore
        }
    }
}

impl From<Message> for LPMsg {
    fn from(msg: Message) -> Self {
        use Message::*;
        match msg {
            LeftPanel(msg) => msg,
            TopMenu(TMMsg::Open) => LPMsg::ShowOpen,
            TopMenu(TMMsg::Save) => LPMsg::ShowSave,
            TopMenu(TMMsg::Hide) | LeftMenu(LMMsg::Hide) => LPMsg::Hide,
            LeftMenu(LMMsg::ShowResize) => LPMsg::ShowResize,
            _ => LPMsg::Ignore,
        }
    }
}

impl From<Message> for RPMsg {
    fn from(msg: Message) -> Self {
        use Message::*;
        match msg {
            RightPanel(msg) => msg,
            RightMenu(RMMsg::ShowBeads) => RPMsg::ShowBeads,
            RightMenu(RMMsg::Hide) => RPMsg::Hide,
            Grid(_) | TopMenu(_) | LeftMenu(_) | LeftPanel(LPMsg::Resize(_,_)) => RPMsg::GridChanged,
            _ => RPMsg::Ignore
        }
    }
}