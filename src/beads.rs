use crate::grid::Grid;
use std::collections::HashMap;
use std::hash::Hash;


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
            BeadsLineBuilder::LROffset(first_offset) => {line_for_offset(table, *first_offset)},
            BeadsLineBuilder::RLOffset(first_offset) => {unimplemented!()},
        }
    }
}

fn to_iter_iter<T: Clone>(table: Vec<&[T]>) -> impl Iterator<Item = impl Iterator<Item=T> + '_> {
    table.into_iter().map(|it|{it.iter().map(Clone::clone)})
}

fn zip_known_first<T: Eq + Hash + Clone>(iter: impl Iterator<Item=T>, first: T) -> BeadsLine<T> {
    let mut variant = first;
    let mut count = 0usize;
    let mut line = Vec::new();

    let mut summary = HashMap::new();
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



fn line_for_square<T: Clone + Eq + Hash>(table: Vec<&[T]>) -> BeadsLine<T> {
    let first = table.get(0).unwrap()[0].clone();
    let iter = table.into_iter().map(|x|x.into_iter()).flatten();
    zip_known_first(iter.map(Clone::clone), first)
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
/*
1|2|3|4|5|6
 2|3|4|5|6|1
2|3|4|5|6|1
 3|4|5|6|1|2
3|4|5|6|1|2
 */

fn line_for_offset<T: Clone + Eq + Hash>(table: Vec<&[T]>, first_offset: bool) -> BeadsLine<T> {
    let first = table.get(0).unwrap()[0].clone();
    let correction = if first_offset { 0 } else { 1 };
    let iter = table.into_iter().enumerate().map(|(i,arr)| {
        let len = arr.len();
        let offset = len - (((i+correction)/2) % len);
        let chunks = vec![&arr[offset..len], &arr[0..offset]];
        chunks.into_iter().map(|x|{ x.iter() }).flatten()
    }).flatten();
    zip_known_first(iter.map(Clone::clone), first)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::beads::line_for_square;

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
        let beads_line = line_for_square(table.table());
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
        let beads = line_for_offset(table.table(), true);
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
        let beads = line_for_offset(table.table(), false);

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