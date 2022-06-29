use std::convert::TryInto;

use super::*;
use super::grid::SimplifiedGrid;
use super::line_builder::BeadsLineBuilder;

impl<T: ColorTrait> Default for Model<T> {
    fn default() -> Self {
        let grid: Grid<_> = Default::default();
        Model::from(grid)
    }
}

#[derive(Clone, Debug)]
pub struct Model<T: ColorTrait> {
    palette: Palette<T>,
    line: BeadsLine<Bead<T>>,
}

fn create_palette<T: ColorTrait>(line: &BeadsLine<Bead<T>>) -> Palette<T> {
    line.map(|Bead{color, ..}|{color.clone()})
        .summary()
        .keys()
        .fold(Palette::new(),|mut palette, item| {
            palette.add_color(item.clone());
            palette
        })
}

impl<T: ColorTrait> From<BeadsLine<Bead<T>>> for Model<T> {
    fn from(line: BeadsLine<Bead<T>>) -> Self {
        let palette = create_palette(&line);
        Model {palette, line}
    }
}

impl <T: ColorTrait> From<Grid<Bead<T>>> for Model<T> {
    fn from(grid: Grid<Bead<T>>) -> Self {
        let builder: BeadsLineBuilder = Schema::default().into();
        let line = builder.build(grid.as_table_iter(), grid.size().width);
        let grid = line.grid();
        let palette = create_palette(&line);
        Model {palette, line }
    }
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
    pub fn add_color(&mut self, color: T) {
        self.palette.add_color(color);
    }
    pub fn activate_color(&mut self, color: T) -> T {
        self.palette.activate(color)
    }
    pub fn remove_color(&mut self) {
        self.palette.remove_color();
    }
    pub fn set_schema(&mut self, schema: Schema) {
        self.line.schema = schema;
    }
    fn simplified_grid(&mut self) -> SimplifiedGrid<Bead<T>> {
        let grid = self.line.grid();
        let grid = unfill_grid(grid);
        grid.simplify()
    }
    pub fn set(&mut self, row: usize, column: usize) -> Result<Option<Bead<T>>, String> {
        let color = self.palette.activated().clone();
        let bead = Bead{ color, filled: false };
        let result =  self.line.set_value(bead.clone(), Coord{x:column, y: row} );
        Ok(result)
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
        let mut grid = self.simplified_grid();
        grid.grow(side, Bead {color: value, filled: false});
        self.update_from_simplified(grid);
    }
    pub fn shrink(&mut self, side: Side) -> Result<(), String>{
        if matches!(side, Side::Top) {
            self.line.shrink_top();
            return Ok(())
        }
        let mut grid = self.simplified_grid();
        grid.shrink(side)?;
        self.update_from_simplified(grid);
        Ok(())
    }
    
    fn update_from_simplified(&mut self, grid: SimplifiedGrid<Bead<T>>) {
        let builder: BeadsLineBuilder = self.line.schema.into();
        self.line = builder.build(
            grid.as_table_iter(), 
            grid.size().width
        );
    }

    pub fn rotate(&mut self, rotation: isize) {
        let mut grid = self.simplified_grid();
        grid.rotate(rotation);
        self.update_from_simplified(grid);
    }
}


fn unfill_grid<T: Debug + Clone + ColorTrait>(grid: Grid<Bead<T>>) -> Grid<Bead<T>> { //TODO: плохой метод, надо его убрать
    grid.map(|Bead { color, ..}|Bead{color: color.clone(), filled: false})
}

impl<T: ColorTrait + Default> Model<T> {
    pub fn resize(&mut self, size: Size) {
        let mut grid = self.simplified_grid();
        grid.resize(size);
        self.update_from_simplified(grid);
    }
}

impl<T: ColorTrait> AsRef<BeadsLine<Bead<T>>> for Model<T> {
    fn as_ref(&self) -> &BeadsLine<Bead<T>> {
        &self.line
    }
}


impl<T: ColorTrait> AsRef<Palette<T>> for Model<T> {
    fn as_ref(&self) -> &Palette<T> {
        &self.palette
    }
}

impl<T: ColorTrait> GetSchema for Model<T> {
    fn get_schema(&self) -> Schema {
        self.line.schema
    }
}