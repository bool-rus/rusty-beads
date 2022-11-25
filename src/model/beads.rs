use indexmap::{IndexSet, IndexMap};

use crate::wrapper::{Uncompressable, Compressable, Chunkable};

use super::{*, grid::SimplifiedGrid};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeadsLine<T: Eq + Hash + Clone + Debug + Default> {
    pub width: usize,
    pub height: usize,
    pub(super) line: Vec<(Bead<T>,usize)>,
    pub schema: Schema,
}

impl<T: Eq + Hash + Clone + Default + Debug> Default for BeadsLine<T> {
    fn default() -> Self {
        let width = 100;
        let height = 100;
        Self { width, height, line: vec![(T::default().into(), width * height)], schema: Default::default() }
    }
}

impl <T: Default + Eq + Hash + Clone + Debug> BeadsLine<T> {
    pub fn resize(&mut self, size: Size) {
        let mut grid = self.simplified_grid();
        grid.resize(size);
        *self = Self::from_simplified_grid(grid, self.schema.clone());
    }

    pub fn change_schema(&mut self, schema: Schema) {
        let grid = self.simplified_grid();
        *self = Self::from_simplified_grid(grid, schema);
    }

    pub fn grow(&mut self, side: Side) {
        match side {
            Side::Top => self.grow_top(),
            side => {
                let mut grid = self.simplified_grid();
                grid.grow(side, T::default());
                *self = Self::from_simplified_grid(grid, self.schema.clone());
            }
        }
    }
    pub fn shrink(&mut self, side: Side) {
        match side {
            Side::Top => self.shrink_top(),
            side => {
                let mut grid = self.simplified_grid();
                grid.grow(side, T::default());
                *self = Self::from_simplified_grid(grid, self.schema.clone());
            }
        }
    }

    pub fn simplified_grid(&self) -> SimplifiedGrid<T> {
        let data = self.table(0)
        .map(|br|br.iter)
        .flatten().map(|(_, obj)|obj.color.clone()).collect();
        SimplifiedGrid::from_raw(NonZeroUsize::new(self.width).unwrap(), data)
    }

    pub fn from_simplified_grid(grid: SimplifiedGrid<T>, schema: Schema) -> Self {
        let width = grid.size().width();
        let height = grid.size().height();
        let line = grid.as_table_iter().enumerate().map(|(n, i)|{
            let rot = schema.calculate_rotation(n, width, 0);
            i.rev().cycle().skip(rot).take(width)
        }).flatten().compress()
        .map(|(obj,count)|(obj.into(), count))
        .collect();
        
        Self {width, height, line, schema}
    }

    pub fn grow_top(&mut self) {
        let default_item = T::default();
        if let Some((first_item, count)) = self.line.first_mut() {
            if &first_item.color == &default_item {
                *count += self.width;
                return;
            }
        } 
        let mut buf = Vec::with_capacity(self.line.len() + 1);
        buf.push((default_item.into(), self.width));
        std::mem::swap(&mut buf, &mut self.line);
        self.line.append(&mut buf);
    }
    pub fn shrink_top(&mut self) {
        let first = self.line.first().unwrap().clone().0;
        let mut buf = vec![(first,1)];
        std::mem::swap(&mut buf, &mut self.line);
        self.line = buf.iter()
        .uncompress().cloned()
        .skip(self.width)
        .compress()
        .collect();
    }
}

pub struct BeadsRow<'a, T> {
    pub row: usize,
    pub offset: usize,
    pub iter: Box<dyn Iterator<Item=(usize, &'a T)> + 'a>,
}

impl<T: Eq + Hash + Clone + Debug + Default + ColorTrait> BeadsLine<T> {
    pub fn width(&self) -> usize {
        self.width
    }
    fn normalize_rotation(&self, rot: isize) -> usize {
        let width = self.width as isize;
        let modulo = rot % width;
        if modulo >= 0 { modulo as usize} else { (width + modulo) as usize }
    }
    pub fn table(&self, rotation: isize) -> impl Iterator<Item=BeadsRow<'_, Bead<T>>> {
        let rotation = self.normalize_rotation(rotation); 
        let width = self.width;
        let schema = self.schema;
        self.line.iter().uncompress().chunks(width).enumerate().map(move |(row_num, chunk)|{
            let rotation = schema.calculate_rotation(row_num, width, rotation);
            let iter = chunk.into_iter().enumerate().rev().cycle().skip(rotation).take(width);
            BeadsRow {row: row_num, offset: schema.calculate_offset(row_num), iter:  Box::new(iter) }
        })
    }
    pub fn set_value(&mut self, value: T, coord: Coord) -> Option<Bead<T>> {
        let index = coord.x + self.width * coord.y;
        let mut buf = Vec::with_capacity(self.line.len() + 2);
        std::mem::swap(&mut buf, &mut self.line);
        let mut result = None;
        let mut iter = buf.into_iter();
        let mut counter = 0;
        while let Some((obj, count)) = iter.next() {
            counter += count;
            if counter <= index {
                self.line.push((obj, count));
                continue;
            } 
            if value == obj.color {
                self.line.push((obj, count));
                break;
            }
            result = Some(obj.clone());
            let first_part = index - (counter - count);
            let second_part = count - first_part - 1;
            if first_part == 0 {
                if let Some((prev, count)) = self.line.last_mut() {
                    if &prev.color == &value {
                        *count += first_part + 1;
                        prev.filled = false;
                    } else {     
                        self.line.push((value.into(), 1));
                    }
                } else {
                    self.line.push((value.into(), 1));
                }
            } else {
                self.line.push((obj.unfill(), first_part));
                self.line.push((value.into(), 1));
            }
            if second_part == 0 {
                if let Some((next, count)) = iter.next() {
                    if let Some((prev, prev_count)) = self.line.last_mut() {
                        if prev == &next {
                            *prev_count += count;
                        } else {
                            self.line.push((next, count));
                        }
                    }
                }
            } else {
                self.line.push((obj.unfill(), second_part));
            }
            break;
        }
        self.line.extend(iter);
        return result;
    }
    pub fn get_mut(&mut self, index: usize) -> Option<&mut Bead<T>> {
        self.line.get_mut(index).map(|(obj, _count)|obj)
    }
    pub fn line(&self) -> &Vec<(Bead<T>, usize)> {
        &self.line
    }
    pub fn line_mut(&mut self) -> &mut Vec<(Bead<T>,usize)> {
        &mut self.line
    }
    pub fn summary(&self) -> IndexMap<T, usize> {
        self.line.iter().fold(IndexMap::default(), |mut summary, (item, count)|{
            if let Some(saved) = summary.get_mut(&item.color) {
                *saved += *count;
            } else {
                summary.insert(item.color.clone(), *count);
            }
            summary
        })
    }
    /* 
    pub fn map<X: Debug + Hash + Eq + Clone + Default, F: Fn(&Bead<T>)->Bead<X>>(&self, fun: F) -> BeadsLine<Bead<X>> {
        BeadsLine {
            width: self.width,
            height: self.height,
            schema: self.schema,
            line: self.line.iter().map(|(x, count)|(fun(x), *count)).collect()
        }
    }*/
}


#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Bead<T: ColorTrait> {
    pub color: T,
    pub filled: bool,
}

impl<T: ColorTrait> Bead<T> {
    fn unfill(&self) -> Self{
        Bead{color: self.color.clone(), filled: false}
    }
}

impl<T: ColorTrait> From<&T> for Bead<T> {
    fn from(color: &T) -> Self {
        Bead{color: color.clone(), filled: false}
    }
}
impl<T: ColorTrait> From<T> for Bead<T> {
    fn from(color: T) -> Self {
        Bead {color, filled:false}
    }
}

impl<T: ColorTrait + Default> Default for Bead<T> {
    fn default() -> Self {
        Bead {color: T::default(), filled: false}
    }
}

#[cfg(test)]
mod test {
    use std::time::Instant;
    use super::*;
    use rand::Rng;

    struct X<T>(Vec<(T, usize)>);

    impl<T> X<T> {
        fn twisted_iter_table(&self, width: usize, revert_chunk: bool) -> impl Iterator<Item=impl Iterator<Item=&T>> {
            self.0.iter().uncompress().chunks(width)
            .enumerate()
            .map(move |(n, chunk)|{
                let iter = chunk.into_iter();
                let x: Box<dyn Iterator<Item=&T>> = if revert_chunk {
                   Box::new( twist_iterator(iter.rev(), n, width) )
                } else {
                    Box::new(twist_iterator(iter, n, width))
                };
                x
            })
        }
    }

    fn twist_iterator<I:Iterator+Clone>(iter: I, n: usize, width: usize) -> impl Iterator<Item = I::Item> {
        iter.cycle().skip(n%width).take(width)
    }

    #[test]
    fn test_twisted() {
        let x = X((1..10).into_iter().map(|x|(x,1)).collect());
        let twisted:Vec<_> = x.twisted_iter_table(3,false).flatten().copied().collect();
        assert_eq!(twisted, vec![1,2,3,5,6,4,9,7,8]);
        let twisted_rev: Vec<_> = x.twisted_iter_table(3,true).flatten().copied().collect();
        assert_eq!(twisted_rev, vec![3,2,1,5,4,6,7,9,8])
    }


    #[test] 
    fn load_test() {
        let v = (0..2000u32).into_iter().map(|x|(x, 100)).collect();
        let x = X(v);
        let start = Instant::now();
        for _ in 0..10 {
            let table = x.twisted_iter_table(5,true);
            let v = table.fold(Vec::new(), |mut v, iter| {
                v.push(
                    iter.fold(Vec::with_capacity(5), |mut v, x|{
                        v.push(*x);
                        v
                    }).len()
                );
                v
            });
            v.len();
        }
        println!("with iters: {:?}", start.elapsed());
        let start = Instant::now();
        for _ in 0..10 {
            let table = x.twisted_iter_table(50,true);
            let v = table.fold(Vec::new(), |mut v, iter| {
                v.push(
                    iter.fold(Vec::with_capacity(50), |mut v, x|{
                        v.push(*x);
                        v
                    }).len()
                );
                v
            });
            v.len();
        }
        println!("with data: {:?}", start.elapsed());
    }

    #[test]
    fn test_from_grid() {
        let width = 40;
        let mut rng = rand::thread_rng();
        let x = (0..(width*width)).into_iter().map(|_|rng.gen_range(0..10u32).into()).compress();
        let line = BeadsLine { width, height: width, line: x.collect(), schema: Default::default() };
        let line_backup =line.clone();
        let grid = line.simplified_grid();
        let line = BeadsLine::from_simplified_grid(grid, Default::default());
        
        assert_eq!(line_backup.line, line.line)

    }

}
