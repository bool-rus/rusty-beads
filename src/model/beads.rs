use crate::wrapper::{Uncompressable, Compressable};

use super::*;
use super::line_builder::BeadsLineBuilder;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeadsLine<T: Eq + Hash + Clone> {
    pub width: usize,
    pub(super) line: Vec<(T,usize)>,
    pub schema: Schema,
}

impl <T: Default + Eq + Hash + Clone + Debug> BeadsLine<T> {
    #[must_use]
    pub fn grow_top(&mut self) -> Grid<T> {
        let default_item = T::default();
        if let Some((first_item, count)) = self.line.first_mut() {
            if first_item == &default_item {
                *count += self.width;
                return self.grid();
            }
        } 
        let mut buf = Vec::with_capacity(self.line.len() + 1);
        buf.push((default_item, self.width));
        std::mem::swap(&mut buf, &mut self.line);
        self.line.append(&mut buf);
        self.grid()
    }
    pub fn shrink_top(&mut self) -> Grid<T> {
        let first = self.line.first().unwrap().clone().0;
        let mut buf = vec![(first,1)];
        std::mem::swap(&mut buf, &mut self.line);
        self.line = buf.into_iter()
        .uncompress()
        .skip(self.width)
        .compress()
        .collect();
        self.grid()
    }
}

impl<T: Eq + Hash + Clone + Debug> BeadsLine<T> {
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
        let builder: BeadsLineBuilder = self.schema.into();
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