use std::num::NonZeroUsize;
use std::hash::Hash;
use std::fmt::Debug;
use serde::{Serialize, Deserialize};

pub mod grid;
mod color;
pub mod beads;
mod faces;
mod model;

pub use model::Model;
pub use faces::*;
pub use grid::Grid;
pub use beads::{Bead, BeadsLine};
pub use color::Color;

#[derive(Debug, Copy, Clone)]
pub enum Side { Top, Left, Right, Bottom }

#[derive(Serialize, Deserialize)]
enum SchemaOld {
    FirstOffset,
    SecondOffset,
    Straight,
}
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum SchemaCompat {
    Old(SchemaOld),
    Actual{base_offset: usize, offset_step: usize},
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[serde(from = "SchemaCompat")]
pub struct Schema {
    base_offset: usize,
    offset_step: usize,
}

impl From<SchemaCompat> for Schema {
    fn from(value: SchemaCompat) -> Self {
        match value {
            SchemaCompat::Old(SchemaOld::Straight) => Self {base_offset: 1, offset_step: 0},
            SchemaCompat::Old(_) => Self {base_offset: 2, offset_step: 1},
            SchemaCompat::Actual { base_offset, offset_step } => Self {base_offset, offset_step},
        }
    }
}

impl Schema {
    pub fn switch(self) -> Self {
        match self {
            Self {base_offset: 1, offset_step: 0} => Self {base_offset: 4, offset_step: 1},
            Self {base_offset: 4, offset_step: 1} => Self {base_offset: 3, offset_step: 1},
            Self {base_offset: 3, offset_step: 1} => Self {base_offset: 7, offset_step: 3},
            Self {base_offset: 7, offset_step: 3} => Self {base_offset: 2, offset_step: 1},
            Self {base_offset: 2, offset_step: 1} => Self {base_offset: 1, offset_step: 0},
            _ => Self {base_offset: 1, offset_step: 0}
        }
    }
    pub fn calculate_rotation(&self, row: usize, width: usize, rotation: usize) -> usize {
        width - (rotation + row*self.offset_step/self.base_offset) % width
    }
    pub fn calculate_offset(&self, row: usize) -> usize {
        row * self.offset_step % self.base_offset
    }
    pub fn base(&self) -> usize {
        self.base_offset
    }
}

impl Default for Schema {
    fn default() -> Self {
        Self {base_offset: 2, offset_step: 1}
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Size {
    pub width: NonZeroUsize,
    pub height: NonZeroUsize,
}

impl Default for Size {
    fn default() -> Self {
        Self {
            width: NonZeroUsize::new(33).unwrap(),
            height: NonZeroUsize::new(33).unwrap(),
        }
    }
}

impl Size {
    pub fn capacity(&self) -> usize {
        self.width.get() * self.height.get()
    }
    pub fn width(&self) -> usize {
        self.width.get()
    }
    pub fn height(&self) -> usize {
        self.height.get()
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
}

pub trait Increasable {
    fn increase(self) -> Self;
}

pub trait Decreasable {
    fn decrease(self) -> Option<Self> where Self: Sized;
}

impl Increasable for NonZeroUsize {
    fn increase(self) -> Self {
        NonZeroUsize::new(self.get() + 1).unwrap()
    }
}

impl Decreasable for NonZeroUsize {
    fn decrease(self) -> Option<Self> {
        NonZeroUsize::new(self.get() - 1)
    }
}