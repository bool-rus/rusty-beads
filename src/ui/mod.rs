extern crate iced_native;
extern crate iced_wgpu;

mod icon;
mod grid;
mod pallette;
mod menu;
mod style;

use crate::reimport::*;
pub use pallette::Pallette;
pub use menu::{TopMenu,RightMenu};

pub trait AsContainer<M> {
    fn as_container(&mut self) -> Container<'_, M>;
}
