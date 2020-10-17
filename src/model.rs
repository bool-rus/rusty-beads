use crate::grid::Grid;
use crate::beads::{BeadsLine, BeadsLineBuilder};
use std::hash::Hash;
use std::fmt::Debug;
use std::mem;
use crate::entities::{Side, Size, Schema};
use std::num::NonZeroUsize;
use serde::{Serialize, Deserialize};

pub trait ColorTrait: Debug + Clone + Hash + Eq + PartialEq {}

impl<T> ColorTrait for T where T: Debug + Clone + Hash + Eq + PartialEq {}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Bead<T: ColorTrait> {
    pub color: T,
    pub filled: bool,
}

impl<T: ColorTrait> From<&T> for Bead<T> {
    fn from(color: &T) -> Self {
        Bead{color: color.clone(), filled: false}
    }
}

impl<T: ColorTrait + Default> Default for Bead<T> {
    fn default() -> Self {
        Bead {color: T::default(), filled: false}
    }
}

impl<T: ColorTrait + Default> Default for Model<T> {
    fn default() -> Self {
        let grid: Grid<_> = Default::default();
        let builder: BeadsLineBuilder = Schema::default().into();
        let line = builder.build(grid.as_table());
        Model {grid, line}
    }
}

#[derive(Clone, Debug)]
pub struct Model<T: ColorTrait> {
    grid: Grid<Bead<T>>,
    line: BeadsLine<Bead<T>>,
}

impl<T: ColorTrait> From<BeadsLine<Bead<T>>> for Model<T> {
    fn from(line: BeadsLine<Bead<T>>) -> Self {
        let grid = line.grid();
        Model {grid, line}
    }
}

impl <T: ColorTrait> From<Grid<Bead<T>>> for Model<T> {
    fn from(grid: Grid<Bead<T>>) -> Self {
        let builder: BeadsLineBuilder = Schema::default().into();
        let line = builder.build(grid.as_table());
        Model {line, grid}
    }
}

impl<T: ColorTrait> Model<T> {
    pub fn width(&self) -> usize {
        self.grid.width()
    }
    pub fn height(&self) -> usize {
        self.grid.height()
    }
    pub fn grid(&self) -> &Grid<Bead<T>> {
        &self.grid
    }
    pub fn line(&self) -> &BeadsLine<Bead<T>> {
        &self.line
    }
    pub fn schema(&self) -> Schema {
        self.line.knit_type
    }
    pub fn set_schema(&mut self, schema: Schema) {
        self.line.knit_type = schema;
        self.unfill_grid();
        self.update_line();
    }
    fn unfill_grid(&mut self) {
        self.grid = self.grid.map(|Bead { color, ..}|Bead{color: color.clone(), filled: false});
    }
    fn update_line(&mut self) {
        let builder: BeadsLineBuilder = self.line.knit_type.into();
        self.line = builder.build(self.grid.as_table());
    }
    pub fn grid_color(&self) -> Grid<T> {
        self.grid.map(|bead|bead.color.clone())
    }
    pub fn line_color(&self) -> BeadsLine<T> {
        self.line.map(|bead|bead.color.clone())
    }
    pub fn set(&mut self, row: usize, column: usize, color: T) -> Result<Option<Bead<T>>, String> {
        let prev = self.grid.get_mut(row, column)?;
        if color.eq(&prev.color) {
            Ok(None)
        } else {
            let mut bead = Bead{ color, filled: false };
            mem::swap(prev, &mut bead);
            if bead.filled {
                self.unfill_grid();
            }
            self.update_line();
            Ok(Some(bead))
        }
    }
    pub fn toggle_filled(&mut self, index: usize) -> Result<bool, String> {
        let obj = self.line.get_mut(index).ok_or("Toggle is out of bounds")?;
        let filled = obj.filled;
        obj.filled = !filled;
        self.grid = self.line.grid();
        Ok(filled)
    }

    pub fn grow(&mut self, side: Side, value: T) {
        self.grid = self.grid.map(|Bead {color, ..}| Bead {color: color.clone(), filled: false});
        let value = Bead {color: value, filled: false};
        self.grid.grow(side, value);
        self.update_line();
    }
    pub fn shrink(&mut self, side: Side) -> Result<(), String>{
        self.grid.shrink(side)?;
        self.unfill_grid();
        self.update_line();
        Ok(())
    }
}

impl<T: ColorTrait + Default> Model<T> {
    pub fn resize(&mut self, size: Size) {
        self.grid.resize(size);
        self.unfill_grid();
        self.update_line();
    }
}