use super::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::{BuildHasher, BuildHasherDefault};

#[derive(Debug, Clone)]
pub struct Palette<T: ColorTrait> {
    colors: HashSet<T, BuildHasherDefault<DefaultHasher>>,
    activated: usize,
}

impl<T: ColorTrait> Palette<T> {
    pub fn new() -> Self {
        let mut colors = HashSet::default();
        colors.insert(T::default());
        Self {colors, activated: 0}
    }
    pub fn activated(&self) -> usize {
        self.activated
    }
    pub fn colors(&self) -> &HashSet<T, BuildHasherDefault<DefaultHasher>> {
        &self.colors
    }
    pub fn add_color(&mut self, color: T) {
        self.colors.insert(color);
    }
}
