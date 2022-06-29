use std::num::NonZeroUsize;
use std::collections::HashMap;
use std::hash::Hash;
use std::fmt::Debug;
use serde::{Serialize, Deserialize};

pub mod grid;
mod color;
pub mod beads;
mod faces;
mod model;
mod line_builder;
mod palette;

pub use faces::*;
pub use grid::Grid;
pub use model::Model;
pub use beads::{Bead, BeadsLine};
pub use color::Color;
pub use palette::Palette;


pub type ColorBead = Bead<Color>;

pub type BeadGrid = Grid<Bead<Color>>;

#[derive(Debug, Copy, Clone)]
pub enum Side { Top, Left, Right, Bottom }

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum Schema {
    FirstOffset,
    SecondOffset,
    Straight,
}

impl Schema {
    pub fn switch(self) -> Self {
        use Schema::*;
        match self {
            FirstOffset => SecondOffset,
            SecondOffset => Straight,
            Straight => FirstOffset,
        }
    }
}

impl Default for Schema {
    fn default() -> Self {
        Schema::SecondOffset
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

#[derive(Debug, Copy, Clone)]
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