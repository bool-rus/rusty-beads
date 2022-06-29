use crate::wrapper::{Uncompressable, Compressable};

use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeadsLine<T: Eq + Hash + Clone> {
    pub width: usize,
    pub(super) line: Vec<(T,usize)>,
    pub schema: Schema,
}

impl <T: Default + Eq + Hash + Clone + Debug> BeadsLine<T> {
    pub fn grow_top(&mut self) {
        let default_item = T::default();
        if let Some((first_item, count)) = self.line.first_mut() {
            if first_item == &default_item {
                *count += self.width;
                return;
            }
        } 
        let mut buf = Vec::with_capacity(self.line.len() + 1);
        buf.push((default_item, self.width));
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
    pub fn data(&self) -> (Vec<&T>, usize) {
        (self.line.iter().uncompress().collect(), self.width)
    }
}

impl<T: Eq + Hash + Clone + Debug> BeadsLine<T> {
    pub fn width(&self) -> usize {
        self.width
    }
    pub fn set_value(&mut self, value: T, coord: Coord) -> Option<T> {
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
            if value == obj {
                self.line.push((obj, count));
                break;
            }
            result = Some(obj.clone());
            let first_part = index - (counter - count);
            let second_part = count - first_part - 1;
            if first_part == 0 {
                if let Some((prev, count)) = self.line.last_mut() {
                    if prev == &value {
                        *count += first_part + 1;
                    } else {     
                        self.line.push((value, 1));
                    }
                } else {
                    self.line.push((value, 1));
                }
            } else {
                self.line.push((obj.clone(), first_part));
                self.line.push((value, 1));
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
                self.line.push((obj, second_part));
            }
            break;
        }
        self.line.extend(iter);
        return result;
    }
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.line.get_mut(index).map(|(obj, _count)|obj)
    }
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
        let builder: line_builder::BeadsLineBuilder = self.schema.into();
        builder.grid(self.width, unzipped)
    }
    pub fn map<X: Debug + Hash + Eq + Clone, F: Fn(&T)->X>(&self, fun: F) -> BeadsLine<X> {
        BeadsLine {
            width: self.width,
            schema: self.schema,
            line: self.line.iter().map(|(x, count)|(fun(x), *count)).collect()
        }
    }
}


#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Bead<T: ColorTrait> {
    pub color: T,
    pub filled: bool,
}

impl<T: ColorTrait> From<&T> for Bead<T> {
    fn from(color: &T) -> Self {
        Bead{color: color.clone(), filled: false}
    }
}

impl<T: ColorTrait + Default> Default for Bead<T> {
    fn default() -> Self {
        Bead {color: T::default(), filled: false}
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_iters() {
        let x = [1,2,3,4,5,6,7,8,9,0];
        let z = x.into_iter();
        let y: Vec<_> = x.into_iter().take(4).collect();
        println!("y: {:?}", y);
    }
    fn test_iterators<T>(data: &[T]) -> Box<dyn Iterator<Item=&T> + '_> {
        if data.len() > 10 {
            Box::new(data.into_iter().rev())
        } else {
            Box::new(data.into_iter())
        }
    }
}
