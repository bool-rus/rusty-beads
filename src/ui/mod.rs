extern crate iced_native;
extern crate iced_wgpu;

mod icon;
mod grid;
mod palette;
mod menu;
mod style;

use crate::reimport::*;
pub use palette::{Message as PaletteMessage};
pub use menu::top::{Message as TopMenuMessage, TopMenu};
pub use menu::right::{Message as RightMenuMessage, RightMenu};
pub use grid::Message as GridMessage;

pub trait AppWidget {
    type Message;
    type UpdateData;
    fn view(&mut self) -> Element<'_, Self::Message>;
    fn update(&mut self, _msg: Self::Message){}
    //TODO: может, от этого лучше отказаться
    fn update_data(&mut self, _data: &Self::UpdateData){}
}
