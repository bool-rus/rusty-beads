mod files;
mod widget;
mod icon;
mod grid;
mod palette;
mod menu;
mod style;
mod panel;

use crate::reimport::*;
pub use palette::{Message as PaletteMessage};
pub use menu::top::{Message as TopMenuMessage, TopMenu};
pub use menu::right::{Message as RightMenuMessage, RightMenu};
pub use menu::left::{Message as LeftMenuMessage, Menu as LeftMenu};
pub use panel::right::{Message as RightPanelMessage, RightPanel, State as RightPanelState};
pub use panel::left::{Message as LeftPanelMessage, Panel as LeftPanel };
pub use grid::{Message as GridMessage, GridPlate};
pub use widget::MouseListener;

pub trait AppWidget {
    type Message;
    fn view(&mut self) -> Element<'_, Self::Message>;
    fn update(&mut self, _msg: Self::Message){}
}
