use std::{num::NonZeroUsize, ops::Sub};
use std::hash::Hash;
use std::fmt::Debug;
use serde::{Serialize, Deserialize};

pub mod grid;
mod color;
pub mod beads;
mod faces;
mod model;
mod schema;

pub use model::Model;
pub use faces::*;
pub use beads::{Bead, BeadsLine};
pub use color::Color;
pub use schema::Schema;

#[derive(Debug, Copy, Clone)]
pub enum Side { Top, Left, Right, Bottom }

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

#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
}

impl Sub for Coord {
    type Output = egui::Vec2;

    fn sub(self, rhs: Self) -> Self::Output {
        let x = self.x as f32 - rhs.x as f32;
        let y = self.y as f32 - rhs.y as f32;
        egui::vec2(x, y)
    }
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