
use std::collections::HashMap;
use std::hash::Hash;
use std::collections::hash_map::RandomState;
use crate::grid::Grid;
use std::fmt::Debug;
use std::num::NonZeroUsize;

#[derive(Debug)]
pub struct BeadsLine<T: Eq + Hash> {
    width: usize,
    line: Vec<(T,usize)>,
    knit_type: BeadsLineBuilder,
}

impl<T: Eq + Hash + Clone + Debug> BeadsLine<T> {
    pub fn line(&self) -> &Vec<(T, usize)> {
        &self.line
    }
    pub fn summary(&self) -> HashMap<T, usize> {
        self.line.iter().fold(HashMap::new(), |mut summary, (item, count)|{
            if let Some(saved) = summary.get_mut(item) {
                *saved += *count;
            } else {
                summary.insert(item.clone(), *count);
            }
            summary
        })
    }
    pub fn grid(&self) -> Grid<T> {
        let capacity = self.line.iter().map(|(_, count)|*count).sum();
        let unzipped = self.line.iter().fold(
            Vec::with_capacity(capacity),
            |mut data,(item, count)| {
                (0..*count).for_each(|_|data.push(item));
                data
            }
        );
        self.knit_type.grid(self.width, unzipped)
    }
}

#[derive(Debug, Copy, Clone)]
pub enum BeadsLineBuilder {
    LRSquare,
    RLSquare,
    LROffset(bool),
    RLOffset(bool),
}

impl BeadsLineBuilder {
    pub fn build<T: Clone + Eq + Hash>(&self, table: Vec<&[T]>) -> BeadsLine<T> {
        let width = table.get(0).map(|row|row.len()).unwrap_or(0);
        let knit_type = *self;
        let mut iter = table.into_iter().map(|line|line.iter().map(|x|x));
        match self {
            BeadsLineBuilder::LRSquare => {
                let line = zip_line(iter.flatten());
                BeadsLine { width, line, knit_type }
            },
            BeadsLineBuilder::RLSquare => {
                let line = zip_line(iter.map(|line|line.rev()).flatten());
                BeadsLine { width, line, knit_type }
            },
            BeadsLineBuilder::LROffset(first_offset) => {
                let line = line_for_offset(iter, *first_offset, width);
                BeadsLine { width, line, knit_type }
            },
            BeadsLineBuilder::RLOffset(first_offset) => {
                let line = line_for_offset(iter.map(|line|line.rev()), !*first_offset, width);
                BeadsLine { width, line, knit_type }
            },
        }
    }
    pub fn grid<T: Clone + Debug>(&self, width: usize, line: Vec<&T>) -> Grid<T> {

        let data = match self {
            BeadsLineBuilder::LRSquare => line.iter().map(|&i|i.clone()).collect(),
            BeadsLineBuilder::RLSquare => line
                .chunks(width)
                .map(|row|row.iter().rev().map(|&i|i.clone()))
                .flatten()
                .collect(),
            BeadsLineBuilder::LROffset(first_offset) => iter_to_grid_data(
                *first_offset,
                width,
                line.chunks(width).map(|row|row.iter().map(|&i|i))
            ),
            BeadsLineBuilder::RLOffset(first_offset) => iter_to_grid_data(
                *first_offset,
                width,
                line.chunks(width).map(|row|row.iter().rev().map(|&i|i))
            ),
        };

        Grid::frow_raw(NonZeroUsize::new(width).unwrap(), data).unwrap()
    }
}

fn iter_to_grid_data<'a, I, I2,  T: 'a + Clone>(first_offset: bool, width: usize, iter: I) -> Vec<T>
    where I: Iterator<Item=I2>, I2: Iterator<Item=&'a T> + Clone  {
    let correction = if first_offset { 0 } else { 1 };
    iter.enumerate()
        .map(|(i, line)| {
            line.cycle().skip((i+correction)/2usize).take(width)
        })
        .flatten()
        .map(|i|i.clone())
        .collect()
}

fn zip_line<'a, T: Eq + Hash + Clone + 'a>(mut iter: impl Iterator<Item=&'a T>)
    -> Vec<(T, usize)> {
    iter.fold(Vec::new(), |mut line, item|{
        if let Some((obj, count)) = line.last_mut() {
            if (&*obj).eq(item) {
                *count += 1;
            } else {
                line.push((item.clone(), 1usize));
            }
        } else {
            line.push((item.clone(), 1usize));
        }
        line
    })
}


fn line_for_offset<'a, T, I, I2>(iter: I, first_offset: bool, width: usize) -> Vec<(T, usize)>
where T: Clone + Eq + Hash + 'a, I: Iterator<Item=I2>, I2: Iterator<Item=&'a T> + Clone {
    let correction = if first_offset { 0 } else { 1 };
    let iter = iter
        .enumerate()
        .map(|(i, line) |{
            let offset = width - (((i+correction)/2) % width);
            line.cycle().skip(offset).take(width)
        })
        .flatten();
    zip_line(iter)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Table(Vec<usize>, usize);

    impl Table {
        fn new(size: usize) -> Self {
            let chunk: Vec<usize> = (0..size).collect();
            let mut vec = Vec::new();
            (0..(size+2)).for_each(|_|{vec.extend(chunk.iter())});
            Self(vec, size)
        }
        fn table(&self)->Vec<&[usize]> {
            self.0.chunks(self.1).collect()
        }
    }

    #[test]
    fn line_square() {
        let n = 7;
        let table = Table::new(n);
        let bline = BeadsLineBuilder::LRSquare.build(table.table());
        let (line, summary) = (bline.line(), bline.summary());
        let height = table.table().len();
        assert_eq!(line.len(), n*height);
        assert_eq!(summary.get(&3),Some(&height));

        let vec: Vec<usize> = line.iter().map(|&(obj, count)|{
            assert_eq!(count, 1);
            obj
        }).collect();

        assert_eq!(bline.grid().as_table(), table.table())
    }

    /*
     1|2|3|4|5|6
                1|2|3|4|5|6
                           6|1|2|3|4|5
                                      6|1|2|3|4|5|
                                                 5|6|1|2|3|4
                                                            5|6|1|2|3|4
                                                                       4|5|6|1|2|3
                                                                                  4|5|6|1|2|3
    */
    #[test]
    fn line_offset() {
        let n = 4;
        let table = Table::new(n);
        let height = table.table().len();
        let bline = BeadsLineBuilder::LROffset(true).build(table.table());
        let (line, summary) = (bline.line(), bline.summary());
        let sum:usize = line.iter()
            .map(|&(i,c)|{c})
            .sum();
        assert_eq!(sum, n*height);
        assert_eq!( line.as_slice() , &[
            (0,1),(1,1),(2,1),(3,1),
            (0,1),(1,1),(2,1),
            (3,2),(0,1),(1,1),(2,1),
            (3,1),(0,1),(1,1),
            (2,2),(3,1),(0,1),(1,1),
            (2,1),(3,1),(0,1),(1,1),
        ]);

        assert_eq!(bline.grid().as_table(), table.table());

        let bline = BeadsLineBuilder::LROffset(false).build(table.table());
        let (line, summary) = (bline.line(), bline.summary());

        assert_eq!( line.as_slice() , &[
            (0,1),(1,1),(2,1),
            (3,2),(0,1),(1,1),(2,1),
            (3,1),(0,1),(1,1),
            (2,2),(3,1),(0,1),(1,1),
            (2,1),(3,1),(0,1),
            (1,2),(2,1),(3,1),(0,1)
        ]);

        assert_eq!(bline.grid().as_table(), table.table())
    }
}