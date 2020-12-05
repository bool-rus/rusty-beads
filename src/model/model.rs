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
    grid: Grid<Bead<T>>,
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
        let grid = line.grid();
        let palette = create_palette(&line);
        Model {palette, grid, line}
    }
}

impl <T: ColorTrait> From<Grid<Bead<T>>> for Model<T> {
    fn from(grid: Grid<Bead<T>>) -> Self {
        let builder: BeadsLineBuilder = Schema::default().into();
        let line = builder.build(grid.as_table_iter(), grid.size().width);
        let grid = line.grid();
        let palette = create_palette(&line);
        Model {palette, line, grid}
    }
}

impl<T: ColorTrait> Model<T> {
    pub fn size(&self) -> Size {
        self.grid.size()
    }
    pub fn grid(&self) -> &Grid<Bead<T>> {
        &self.grid
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
        self.unfill_grid();
        self.update_line();
        self.grid = self.line.grid();
    }
    fn unfill_grid(&mut self) { //TODO: плохой метод, надо его убрать
        self.grid = self.grid.map(|Bead { color, ..}|Bead{color: color.clone(), filled: false});
    }
    fn update_line(&mut self) {
        let builder: BeadsLineBuilder = self.line.schema.into();
        self.line = builder.build(self.grid.as_table_iter(), self.grid.size().width);
    }
    fn simplified_grid(&mut self) -> SimplifiedGrid<Bead<T>> {
        self.unfill_grid();
        self.grid.simplify()
    }
    pub fn set(&mut self, row: usize, column: usize) -> Result<Option<Bead<T>>, String> {
        let color = self.palette.activated().clone();
        let prev = self.grid.get_mut(row, column)?;
        if color.eq(&prev.color) {
            Ok(None)
        } else {
            let mut bead = Bead{ color, filled: false };
            core::mem::swap(prev, &mut bead);
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
        let mut grid = self.simplified_grid();
        grid.grow(side, Bead {color: value, filled: false});
        self.update_from_simplified(grid);
    }
    pub fn shrink(&mut self, side: Side) -> Result<(), String>{
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
        self.grid = self.line.grid();
    }

    pub fn rotate(&mut self, rotation: isize) {
        let mut grid = self.simplified_grid();
        grid.rotate(rotation);
        self.update_from_simplified(grid);
    }
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

impl<T: ColorTrait> AsRef<Grid<Bead<T>>> for Model<T> {
    fn as_ref(&self) -> &Grid<Bead<T>> {
        &self.grid
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