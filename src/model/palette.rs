use super::*;
use fxhash::FxHashMap;

#[derive(Debug, Clone)]
pub struct Palette<T: ColorTrait> {
    colors: FxHashMap<T, bool>,
    activated: T,
}

impl<T: ColorTrait> Palette<T> {
    pub fn new() -> Self {
        let mut colors = FxHashMap::default();
        colors.insert(T::default(), true);
        Self {colors, activated: T::default()}
    }
    pub fn activated(&self) -> &T {
        &self.activated
    }
    pub fn colors(&self) -> &FxHashMap<T, bool> {
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
