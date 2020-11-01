use super::*;

pub trait ColorTrait: Debug + Clone + Hash + Eq + PartialEq + Default {}

pub trait GetSchema {
    fn get_schema(&self) -> Schema;
}

impl<T> ColorTrait for T where T: Debug + Clone + Hash + Eq + PartialEq + Default {}