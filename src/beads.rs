use crate::grid::Grid;
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug)]
pub struct BeadsLine<T: Eq + Hash> {
    line: Vec<(T,usize)>,
    summary: HashMap<T, usize>,
}

impl<T: Eq + Hash> BeadsLine<T> {
    fn new(line: Vec<(T,usize)>, summary: HashMap<T, usize>) -> Self {
        Self {line, summary}
    }
    pub fn line(&self) -> &Vec<(T, usize)> {
        &self.line
    }
    pub fn summary(&self) -> &HashMap<T, usize> {
        &self.summary
    }
}

pub enum BeadsLineBuilder {
    LRSquare,
    RLSquare,
    LROffset(bool),
    RLOffset(bool),
}

impl BeadsLineBuilder {
    pub fn build<T: Clone + Eq + Hash>(&self, table: Vec<&[T]>) -> BeadsLine<T> {
        match self {
            BeadsLineBuilder::LRSquare => {unimplemented!()},
            BeadsLineBuilder::RLSquare => {unimplemented!()},
            BeadsLineBuilder::LROffset(first_offset) => {
                let iter = to_iter_iter(table);
                line_for_offset(iter, *first_offset)
            },
            BeadsLineBuilder::RLOffset(first_offset) => {
                let iter = to_iter_rev_iter(table);
                line_for_offset(iter, !*first_offset)
            },
        }
    }
}

fn to_iter_iter<T: Clone>(table: Vec<&[T]>) -> impl Iterator<Item = impl Iterator<Item=T> + '_> {
    table.into_iter().map(|it|{it.iter().map(Clone::clone)})
}
fn to_iter_rev_iter<T: Clone>(table: Vec<&[T]>) -> impl Iterator<Item = impl Iterator<Item=T> + '_> {
    table.into_iter().map(|it|{it.iter().rev().map(Clone::clone)})
}

fn zip_line<T: Eq + Hash + Clone>(mut iter: impl Iterator<Item=T>) -> BeadsLine<T> {
    let first;
    if let Some(item) = iter.next() {
        first = item;
    } else { return BeadsLine::new(Vec::new(),HashMap::new()) }
    let mut variant = first.clone();
    let mut count = 1usize;
    let mut line = Vec::new();
    let mut summary = HashMap::new();
    summary.insert(first, 1usize);
    for item in iter {
        if let Some(stat) = summary.get_mut(&item) {
            *stat += 1;
        } else {
            summary.insert(item.clone(), 1usize);
        }
        if &item == &variant {
            count += 1;
        } else  {
            line.push((std::mem::replace(&mut variant, item), count));
            count = 1;
        }
    }
    line.push((variant, count));
    BeadsLine::new(line, summary)
}



fn line_for_square<T: Clone + Eq + Hash>(iter: impl Iterator<Item = impl Iterator<Item=T>>) -> BeadsLine<T> {
    let iter = iter.map(|x|x.into_iter()).flatten();
    zip_line(iter)
}

fn line_for_offset<T: Clone + Eq + Hash>(iter: impl Iterator<Item = impl Iterator<Item=T>>, first_offset: bool) -> BeadsLine<T> {
    let correction = if first_offset { 0 } else { 1 };
    let iter = iter
        .map(|iter|{iter.collect::<Vec<_>>()})
        .enumerate()
        .map(|(i,arr)| {
            let arr = arr.as_slice();
            let len = arr.len();
            let offset = len - (((i+correction)/2) % len);
            let mut res = Vec::with_capacity(len);
            res.extend_from_slice(&arr[offset..len]);
            res.extend_from_slice(&arr[0..offset]);
            res.into_iter()
    }).flatten();
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
        let beads_line = line_for_square(to_iter_iter(table.table()));
        let height = table.table().len();
        assert_eq!(beads_line.line().len(), n*height);
        assert_eq!(beads_line.summary().get(&3),Some(&height));

        let vec: Vec<usize> = beads_line.line().into_iter().map(|&(obj, count)|{
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
        let beads = line_for_offset(to_iter_iter(table.table()), true);
        let sum:usize = beads.line().iter()
            .map(|&(i,c)|{c})
            .sum();
        assert_eq!(sum, n*height);
        assert_eq!( beads.line().as_slice() , &[
            (0,1),(1,1),(2,1),(3,1),
            (0,1),(1,1),(2,1),
            (3,2),(0,1),(1,1),(2,1),
            (3,1),(0,1),(1,1),
            (2,2),(3,1),(0,1),(1,1),
            (2,1),(3,1),(0,1),(1,1),
        ]);
        let beads = line_for_offset(to_iter_iter(table.table()), false);

        assert_eq!( beads.line().as_slice() , &[
            (0,1),(1,1),(2,1),
            (3,2),(0,1),(1,1),(2,1),
            (3,1),(0,1),(1,1),
            (2,2),(3,1),(0,1),(1,1),
            (2,1),(3,1),(0,1),
            (1,2),(2,1),(3,1),(0,1)
        ]);
    }
}