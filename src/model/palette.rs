use super::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::{BuildHasher, BuildHasherDefault};

#[derive(Debug, Clone)]
pub struct Palette<T: ColorTrait> {
    colors: HashMap<T, bool,  BuildHasherDefault<DefaultHasher>>,
    activated: T,
}

impl<T: ColorTrait> Palette<T> {
    pub fn new() -> Self {
        let mut colors = HashMap::default();
        colors.insert(T::default(), true);
        Self {colors, activated: T::default()}
    }
    pub fn activated(&self) -> &T {
        &self.activated
    }
    pub fn colors(&self) -> &HashMap<T, bool, BuildHasherDefault<DefaultHasher>> {
        &self.colors
    }
    pub fn add_color(&mut self, color: T) {
        self.colors.insert(color, false);
    }
    pub fn activate(&mut self, color: T) -> T {
        let prev = self.activated.clone();
        self.colors.insert(prev.clone(), false);
        self.colors.insert(color.clone(), true);
        self.activated = color;
        return prev
    }
    pub fn remove_color(&mut self) {
        if self.colors.len() > 1 {
            self.colors.remove(&self.activated);
            self.colors.insert(T::default(), true);
            self.activated = T::default();
        }
    }
}
