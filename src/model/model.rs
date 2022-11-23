use std::convert::TryInto;

use super::*;

impl<T: ColorTrait> Default for Model<T> {
    fn default() -> Self {
        //let grid: Grid<_> = Default::default();
        todo!()//Model::from(grid)
    }
}

#[derive(Clone, Debug)]
pub struct Model<T: ColorTrait> {
    line: BeadsLine<Bead<T>>,
}

impl<T: ColorTrait> Model<T> {
    pub fn size(&self) -> Size {
        let data_len: usize = self.line.line.iter().map(|(_,count)|*count).sum();
        let width = self.line.width().try_into().unwrap();
        let height = (data_len/self.line.width).try_into().unwrap();
        Size { width, height }
    }
    pub fn line(&self) -> &BeadsLine<Bead<T>> {
        &self.line
    }
    pub fn schema(&self) -> Schema {
        self.line.schema
    }
    pub fn set_schema(&mut self, schema: Schema) {
        self.line.schema = schema;
    }
    pub fn toggle_filled(&mut self, index: usize) -> Result<bool, String> {
        let obj = self.line.get_mut(index).ok_or("Toggle is out of bounds")?;
        let filled = obj.filled;
        obj.filled = !filled;
        Ok(filled)
    }
    pub fn grow(&mut self, side: Side, value: T) {
        if matches!(side, Side::Top) {
            self.line.grow_top();
            return
        }
        let mut grid = self.line.simplified_grid();
        grid.grow(side, Bead {color: value, filled: false});
        self.line = BeadsLine::from_simplified_grid(grid, self.schema());
    }
    pub fn shrink(&mut self, side: Side) -> Result<(), String>{
        if matches!(side, Side::Top) {
            self.line.shrink_top();
            return Ok(())
        }
        let mut grid = self.line.simplified_grid();
        grid.shrink(side)?;
        self.line = BeadsLine::from_simplified_grid(grid, self.schema());
        Ok(())
    }
    
    pub fn rotate(&mut self, rotation: isize) {
        let mut grid = self.line.simplified_grid();
        grid.rotate(rotation);
        self.line = BeadsLine::from_simplified_grid(grid, self.schema());
    }
}


impl<T: ColorTrait + Default> Model<T> {
    pub fn resize(&mut self, size: Size) {
        let mut grid = self.line.simplified_grid();
        grid.resize(size);
        self.line = BeadsLine::from_simplified_grid(grid, self.schema());
    }
}

impl<T: ColorTrait> AsRef<BeadsLine<Bead<T>>> for Model<T> {
    fn as_ref(&self) -> &BeadsLine<Bead<T>> {
        &self.line
    }
}

impl<T: ColorTrait> GetSchema for Model<T> {
    fn get_schema(&self) -> Schema {
        self.line.schema
    }
}