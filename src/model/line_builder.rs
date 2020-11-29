use super::*;

#[derive(Debug, Copy, Clone)]
pub enum BeadsLineBuilder {
    LRSquare,
    RLSquare,
    LROffset(bool),
    RLOffset(bool),
}

impl From<Schema> for BeadsLineBuilder {
    fn from(schema: Schema) -> Self {
        use Schema::*;
        use BeadsLineBuilder::*;
        match schema{
            FirstOffset => RLOffset(true),
            SecondOffset => RLOffset(false),
            Straight => RLSquare,
        }
    }
}

impl Into<Schema> for BeadsLineBuilder {
    fn into(self) -> Schema {
        use Schema::*;
        use BeadsLineBuilder::*;
        match self {
            RLOffset(true) => FirstOffset,
            RLOffset(false) => SecondOffset,
            RLSquare => Straight,
            _ => unimplemented!(),
        }
    }
}

impl BeadsLineBuilder {
    pub fn build<'a, T, I1, I2>(&self, iter: I1, width: NonZeroUsize) -> BeadsLine<T> 
    where T: Clone + Eq + Hash + 'a, I2: DoubleEndedIterator<Item = &'a T> + Clone, I1: Iterator<Item=I2> {
        let width = width.get();
        let knit_type = (*self).into();
        match self {
            BeadsLineBuilder::LRSquare => {
                let line = zip_line(iter.flatten());
                BeadsLine { width, line, schema: knit_type }
            },
            BeadsLineBuilder::RLSquare => {
                let line = zip_line(iter.map(|line|line.rev()).flatten());
                BeadsLine { width, line, schema: knit_type }
            },
            BeadsLineBuilder::LROffset(first_offset) => {
                let line = line_for_offset(iter, *first_offset, width);
                BeadsLine { width, line, schema: knit_type }
            },
            BeadsLineBuilder::RLOffset(first_offset) => {
                let line = line_for_offset(iter.map(|line|line.rev()), !*first_offset, width);
                BeadsLine { width, line, schema: knit_type }
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
    let correction = if first_offset { 1 } else { 0 };
    iter.enumerate()
        .map(|(i, line)| {
            line.cycle().skip(
                width - (((i+correction)/2) % width)
            ).take(width)
        })
        .flatten()
        .map(|i|i.clone())
        .collect()
}

fn zip_line<'a, T: Eq + Hash + Clone + 'a>(iter: impl Iterator<Item=&'a T>)
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
        fn width(&self) -> NonZeroUsize {
            NonZeroUsize::new(self.1).unwrap()
        }
        fn table(&self)-> impl Iterator<Item=impl DoubleEndedIterator<Item=&usize> + Clone> {
            self.0.chunks(self.1).map(|x|x.into_iter()).map(|x|x)
        }
    }

    fn assert_eq_iters<'a, T, L, R, IL, IR>(left: L, right: R ) 
    where L: Iterator<Item=IL>, IL: Iterator<Item=&'a T>,
    R: Iterator<Item=IR>, IR: Iterator<Item=&'a T>,
    T: 'a + Eq + Debug {
        left.zip(right)
        .for_each(|(left, right)|{
            left.zip(right)
            .for_each(|(left, right)|{
                assert_eq!(left, right)
            })
        })
    }

    #[test]
    fn line_square() {
        let n = 7;
        let table = Table::new(n);
        let bline = BeadsLineBuilder::RLSquare.build(table.table(), table.width());
        let (line, summary) = (bline.line(), bline.summary());
        let height = table.table().count();
        assert_eq!(line.len(), n*height);
        assert_eq!(summary.get(&3),Some(&height));

        let vec: Vec<usize> = line.iter().map(|&(obj, count)|{
            assert_eq!(count, 1);
            obj
        }).collect();

        assert_eq_iters(bline.grid().as_table_iter(), table.table())
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
        let height = table.table().count();
        let bline = BeadsLineBuilder::RLOffset(true).build(table.table(), table.width());
        let (line, summary) = (bline.line(), bline.summary());
        let sum:usize = line.iter()
            .map(|&(i,c)|{c})
            .sum();
        assert_eq!(sum, n*height);
        assert_eq!( line.as_slice() , &[
            (3, 1), (2, 1), (1, 1), (0, 2),
            (3, 1), (2, 1), (1, 1),
            (0, 1), (3, 1), (2, 1), (1, 2),
            (0, 1), (3, 1), (2, 1),
            (1, 1), (0, 1), (3, 1), (2, 2),
            (1, 1), (0, 1), (3, 1),
        ]);

        assert_eq_iters(bline.grid().as_table_iter(), table.table());

        let bline = BeadsLineBuilder::RLOffset(false).build(table.table(), table.width());
        let (line, summary) = (bline.line(), bline.summary());

        assert_eq!( line.as_slice() , &[
            (3, 1), (2, 1), (1, 1), (0, 1),
            (3, 1), (2, 1), (1, 1), (0, 2),
            (3, 1), (2, 1), (1, 1),
            (0, 1), (3, 1), (2, 1), (1, 2),
            (0, 1), (3, 1), (2, 1),
            (1, 1), (0, 1), (3, 1), (2, 1),
        ]);

        assert_eq_iters(bline.grid().as_table_iter(), table.table())
    }
}