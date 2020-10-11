use crate::entities::Color;
use crate::grid::Grid;
use crate::beads::BeadsLine;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Bead {
    pub color: Color,
    pub filled: bool,
}

pub struct Model {
    pub grid: Grid<Bead>,
    pub line: BeadsLine<Bead>,
}

impl Model {
    pub fn grid_color(&self) -> Grid<Color> {
        self.grid.map(|bead|bead.color.clone())
    }
    pub fn line_color(&self) -> BeadsLine<Color> {
        self.line.map(|bead|bead.color.clone())
    }
}