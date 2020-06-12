extern crate iced_native;
extern crate iced_wgpu;

mod icon;
mod grid;
mod palette;
mod menu;
mod style;

use crate::reimport::*;
pub use palette::{Message as PaletteMessage, Palette};
pub use menu::top::{Message as TopMenuMessage, TopMenu};
pub use menu::right::{Message as RightMenuMessage, RightMenu};
pub use grid::Message as GridMessage;

pub trait AsContainer<M> {
    fn as_container(&mut self) -> Container<'_, M>;
}
