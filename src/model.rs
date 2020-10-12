use crate::grid::Grid;
use crate::beads::BeadsLine;
use std::hash::Hash;
use std::fmt::Debug;

pub trait ColorTrait: Debug + Clone + Hash + Eq + PartialEq {}

impl<T> ColorTrait for T where T: Debug + Clone + Hash + Eq + PartialEq {}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Bead<T: ColorTrait> {
    pub color: T,
    pub filled: bool,
}

pub struct Model<T: ColorTrait> {
    pub grid: Grid<Bead<T>>,
    pub line: BeadsLine<Bead<T>>,
}

impl<T: ColorTrait> Model<T> {
    pub fn grid_color(&self) -> Grid<T> {
        self.grid.map(|bead|bead.color.clone())
    }
    pub fn line_color(&self) -> BeadsLine<T> {
        self.line.map(|bead|bead.color.clone())
    }
}