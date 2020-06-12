extern crate iced_native;
extern crate iced_wgpu;

mod icon;
mod grid;
mod pallette;
mod menu;
mod style;

use crate::reimport::*;
pub use pallette::{Message as PaletteMessage, Pallette};
pub use menu::top::{Message as TopMenuMessage, TopMenu};
pub use menu::right::{Message as RightMenuMessage, RightMenu};
pub use grid::Message as GridMessage;

pub trait AsContainer<M> {
    fn as_container(&mut self) -> Container<'_, M>;
}
