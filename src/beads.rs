use crate::lib::Color;
use crate::field::Grid;
use std::io::IntoInnerError;

pub type Beads<T> = Vec<(T, usize)>;

struct BeadsBuilder<T> {
    data: Vec<(T,usize)>,
    item: T,
    count: usize,
}

impl<T: Clone + Eq> Into<Beads<T>> for &Grid<T> {
    fn into(self) -> Beads<T> {
        let data = self.as_data();
        let mut builder = BeadsBuilder::new(data[0].clone());
        &data[1..].iter().for_each(|item|{ builder.add(item.clone()) });
        builder.into()
    }
}

impl<T> Into<Beads<T>> for BeadsBuilder<T> {
    fn into(self) -> Beads<T> {
        let BeadsBuilder {mut data, item, count} = self;
        data.push((item, count));
        data
    }
}

impl<T: Eq> BeadsBuilder<T> {
    fn new(item: T) -> Self {
        Self {
            data: Vec::new(),
            item,
            count: 1,
        }
    }
    fn add(&mut self, item: T) {
        if self.item == item {
            self.count += 1
        } else {
            let finished_item = std::mem::replace(&mut self.item,item);
            self.data.push((finished_item, self.count));
            self.count = 1;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::field::*;
    use crate::lib::Color;
    use std::num::NonZeroUsize;
    use crate::beads::Beads;

    fn grid() -> Grid<Color> {
        Grid::new(
            NonZeroUsize::new(3).unwrap(),
            NonZeroUsize::new(3).unwrap(),
            Color{r:0,g:0,b:0}
        )
    }
    #[test]
    fn smoke() {
        let mut grid = grid();
        grid.set(1,1,Color{r: 10, g: 10, b: 10});
        let beads: Beads<Color> = (&grid).into();
        assert_eq!(beads.len(), 3);
        assert_eq!(beads.get(1).unwrap().1, 1);
    }

    #[test]
    fn check_count() {
        let mut grid = grid();
        grid.set(0,0, Color{r:1, g:1, b:1});
        let beads: Beads<Color> = (&grid).into();
        assert_eq!(beads.len(),2);
        assert_eq!(beads.get(1).unwrap().1, 8);
    }

}