use super::*;
use crate::ui::{
    GridMessage as GMsg,
    TopMenuMessage as TMMsg,
    LeftPanelMessage as LPMsg,
    FilesMessage as FMsg,
    RightPanelMessage as RPMsg,
    LeftMenuMessage as LMMsg,
};

impl From<Message> for GridServiceMessage<Color> {
    fn from(msg: Message) -> Self {
        use Message::*;
        use GridServiceMessage as GSMsg;
        match msg {
            SetGridColor(coord, color) => GSMsg::Point(coord, color),
            LeftPanel(LPMsg::Grow(side)) => GSMsg::Grow(side),
            LeftPanel(LPMsg::Shrink(side)) => GSMsg::Shrink(side),
            LeftPanel(LPMsg::Resize(size)) => GSMsg::Resize(size),
            LeftMenu(LMMsg::SchemaChange) => GSMsg::SchemaChange,
            RightPanel(RPMsg::ToggleCheckbox(index)) => GSMsg::ToggleLineItem(index),
            TopMenu(TMMsg::Undo) => GSMsg::Undo,
            TopMenu(TMMsg::Redo) => GSMsg::Redo,
            _ => GSMsg::Ignore
        }
    }
}

impl From<GridServiceMessage<Color>> for Message {
    fn from(msg: GridServiceMessage<Color>) -> Self {
        use GridServiceMessage::*;
        match msg {
            Updated(v) => Message::GridUpdated(v),
            Loaded(v) => Message::GridLoaded(v),
            _ => Message::Ignore
        }
    }
}

impl From<Message> for IOMessage {
    fn from(msg: Message) -> Self {
        use Message::*;
        match msg {
            LeftPanel(LPMsg::FS(FMsg::Open(path))) => IOMessage::Open(path),
            LeftPanel(LPMsg::FS(FMsg::Save(path))) => IOMessage::Save(path),
            _=> IOMessage::Ignore
        }
    }
}

impl From<IOMessage> for GridServiceMessage<Color> {
    fn from(msg: IOMessage) -> Self {
        use GridServiceMessage::*;
        match msg {
            IOMessage::Loaded(grid) => Loaded(grid),
            _=> Ignore,
        }
    }
}

impl From<GridServiceMessage<Color>> for IOMessage {
    fn from(msg: GridServiceMessage<Color>) -> Self {
        match msg {
            GridServiceMessage::Updated(grid) => IOMessage::GridUpdated(grid),
            _ => IOMessage::Ignore,
        }
    }
}