
use std::collections::HashMap;
use std::hash::Hash;
use std::collections::hash_map::RandomState;
use crate::grid::Grid;

#[derive(Debug)]
pub struct BeadsLine<T: Eq + Hash> {
    width: usize,
    line: Vec<(T,usize)>,
    summary: HashMap<T, usize>,
    knit_type: BeadsLineBuilder,
}

impl<T: Eq + Hash + Clone> BeadsLine<T> {
    pub fn line(&self) -> &Vec<(T, usize)> {
        &self.line
    }
    pub fn summary(&self) -> &HashMap<T, usize> {
        &self.summary
    }
    pub fn grid(&self) -> () {
        let data = self.line.iter().fold(
            Vec::with_capacity(self.summary.values().sum()),
            |mut data,(item, count)| {
                (0..*count).for_each(|_|data.push(item));
                data
            }
        );
        let table = data.as_slice()
            .chunks(self.width)
            .map(|line|line.iter().rev())
            .enumerate()
            .map(|(counter, line)| {
                line.cycle().skip(counter/2usize).take(self.width)
            });
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
                let (line, summary) = zip_line(iter.flatten());
                BeadsLine { width, line, summary, knit_type }
            },
            BeadsLineBuilder::RLSquare => {
                let (line,summary) = zip_line(iter.map(|line|line.rev()).flatten());
                BeadsLine { width, line, summary, knit_type }
            },
            BeadsLineBuilder::LROffset(first_offset) => {
                let (line, summary) = line_for_offset(iter, *first_offset, width);
                BeadsLine { width, line, summary, knit_type }
            },
            BeadsLineBuilder::RLOffset(first_offset) => {
                let (line, summary) = line_for_offset(iter.map(|line|line.rev()), !*first_offset, width);
                BeadsLine { width, line, summary, knit_type }
            },
        }
    }
}

fn zip_line<'a, T: Eq + Hash + Clone + 'a>(mut iter: impl Iterator<Item=&'a T>)
    -> (Vec<(T, usize)>, HashMap<T, usize>) {
    iter.fold((Vec::new(), HashMap::new()), |(mut line, mut summary), item|{
        if let Some((obj, count)) = line.last_mut() {
            if (&*obj).eq(item) {
                *count += 1;
            } else {
                line.push((item.clone(), 1usize));
            }
            if let Some(count) = summary.get_mut(item) {
                *count += 1;
            } else  {
                summary.insert(item.clone(), 1usize);
            }
        } else {
            line.push((item.clone(), 1usize));
            summary.insert(item.clone(), 1usize);
        }
        (line, summary)
    })
}


fn line_for_offset<'a, T, I, I2>(iter: I, first_offset: bool, width: usize) -> (Vec<(T, usize)>, HashMap<T, usize>)
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
        let BeadsLine{line, summary, ..} = BeadsLineBuilder::LRSquare.build(table.table());
        let height = table.table().len();
        assert_eq!(line.len(), n*height);
        assert_eq!(summary.get(&3),Some(&height));

        let vec: Vec<usize> = line.iter().map(|&(obj, count)|{
            assert_eq!(count, 1);
            obj
        }).collect();
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
        let BeadsLine{line, summary, ..} = BeadsLineBuilder::LROffset(true).build(table.table());
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

        let BeadsLine{line, summary, ..} = BeadsLineBuilder::LROffset(false).build(table.table());

        assert_eq!( line.as_slice() , &[
            (0,1),(1,1),(2,1),
            (3,2),(0,1),(1,1),(2,1),
            (3,1),(0,1),(1,1),
            (2,2),(3,1),(0,1),(1,1),
            (2,1),(3,1),(0,1),
            (1,2),(2,1),(3,1),(0,1)
        ]);
    }
}