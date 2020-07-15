use crate::ui::{
    GridMessage as GMsg,
    RightMenuMessage as RMMsg,
    TopMenuMessage as TMMsg,
    RightPanelMessage as RPMsg,
    LeftMenuMessage as LMMsg,
    LeftPanelMessage as LPMsg,
};
use crate::entities::Color;
use std::sync::Arc;
use crate::grid::Grid;

#[derive(Debug, Clone)]
pub enum Message {
    Ignore,
    Grid(GMsg),
    RightMenu(RMMsg),
    TopMenu(TMMsg),
    LeftPanel(LPMsg),
    RightPanel(RPMsg),
    LeftMenu(LMMsg),
    GridUpdated(Arc<Grid<Color>>),
    Error(String),
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
        use crate::ui::FilesMessage;
        match msg {
            Message::TopMenu(msg) => msg,
            Message::LeftMenu(LMMsg::Hide) |
            Message::LeftMenu(LMMsg::ShowResize) => TMMsg::Hide,
            Message::LeftPanel(LPMsg::FS(FilesMessage::Open(..))) |
            Message::LeftPanel(LPMsg::FS(FilesMessage::Save(..))) => TMMsg::Hide,
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
            GridUpdated(v) => GMsg::GridUpdated(v),
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
            GridUpdated(grid) => LPMsg::GridUpdated(grid),
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
            GridUpdated(grid) => RPMsg::GridUpdated(grid),
            LeftMenu(LMMsg::SchemaChange) => RPMsg::Refresh,
            _ => RPMsg::Ignore
        }
    }
}
