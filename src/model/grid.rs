use super::*;

#[derive(Debug)]
pub enum Error {
    InvalidDataSize,
}

impl ToString for Error {
    fn to_string(&self) -> String {
        "Invalid data size".to_string()
    }
}

#[derive(Debug, Clone)]
pub struct Grid<T: Debug + Clone> {
    width: usize,
    height: usize,
    data: Vec<(T, bool)>,
}

impl<T: Debug + Clone> Grid<T> {
    pub fn new(size: Size, item: T) -> Self {
        let width = size.width.get();
        let height = size.height.get();
        Self {
            width,
            height,
            data: vec![(item, false); width*height], //TODO: возможно, стоит переделать
        }
    }
    pub fn frow_raw(width: NonZeroUsize, data: Vec<(T, bool)>) -> Result<Self, Error>{
        let width = width.get();
        if data.len() % width > 0 {
            Err(Error::InvalidDataSize)
        } else {
            let height = data.len()/width;
            Ok(Self { width, height, data })
        }
    }
    pub fn size(&self) -> Size {
        Size {
            width: NonZeroUsize::new(self.width).unwrap(),
            height: NonZeroUsize::new(self.height).unwrap(),
        }
    }
    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.height
    }
    pub fn get_mut(&mut self, row: usize, column: usize) -> Result<&mut T, String> {
        Ok(self.data
            .as_mut_slice()
            .chunks_mut(self.width)
            .nth(row)
            .ok_or("row out of bounds")?
            .get_mut(column)
            .map(|x|&mut x.0)
            .ok_or("column out of bounds")?)
    }
    pub fn as_table_iter(&self) -> impl Iterator<Item=impl DoubleEndedIterator<Item=&T> + Clone> {
        self.data.as_slice()
        .chunks(self.width)
        .map(
            |chunk|chunk.into_iter().map(|x|&x.0)
        )
    }
    pub fn as_full_table_iter(&self) -> impl Iterator<Item=impl DoubleEndedIterator<Item=&(T, bool)> + Clone> {
        self.data.as_slice()
        .chunks(self.width)
        .map(IntoIterator::into_iter)
    }
    fn decreased_height(&self) -> Result<usize, String> {
        if self.height < 2 {
            Err("Cannot remove row".to_owned())
        } else {
            Ok(self.height - 1)
        }
    }
    fn decreased_width(&self) -> Result<usize, String> {
        if self.width < 2 {
            Err("Cannot remove column".to_owned())
        } else {
            Ok(self.width - 1)
        }
    }
    pub fn grow(&mut self, side: Side, value: T) {
        let value = (value, false);
        match side {
            Side::Top => {
                self.height += 1;
                let mut data = Vec::with_capacity(self.data.len() + self.width);
                for _ in 0..self.width {
                    data.push(value.clone())
                }
                data.extend_from_slice(self.data.as_slice());
                self.data = data;
            },
            Side::Left | Side::Right => {
                let width = self.width + 1;
                let mut data = Vec::with_capacity(self.height * width);
                match side {
                    Side::Left => self.as_full_table_iter().for_each(|row| {
                        data.push(value.clone());
                        data.extend(row.map(Clone::clone));
                    }),
                    Side::Right => self.as_full_table_iter().for_each(|row|{
                        data.extend(row.map(Clone::clone));
                        data.push(value.clone());
                    }),
                    _ => {unreachable!()},
                };
                self.width = width;
                self.data = data;
            },
            Side::Bottom => {
                self.height += 1;
                let delta = self.height*self.width - self.data.len();
                self.data.reserve_exact(delta);
                for _ in 0..self.width {
                    self.data.push(value.clone());
                }
            },
        }
    }
    pub fn shrink(&mut self, side: Side) -> Result<(), String> {
        match side {
            Side::Top => {
                self.height = self.decreased_height()?;
                self.data = self.data.iter().skip(self.width).map(Clone::clone).collect();
                Ok(())
            },
            Side::Left | Side::Right => {
                let width = self.decreased_width()?;
                let range = match side {
                    Side::Left => 1..self.width,
                    Side::Right => 0..width,
                    _ => unreachable!(),
                };
                let mut data = Vec::with_capacity(width * self.height);
                data.extend(self.data.chunks(width).map(|row| {
                    (&row[range.clone()]).iter().map(Clone::clone)
                }).flatten());
                self.width = width;
                self.data = data;
                Ok(())
            },
            Side::Bottom => {
                self.height = self.decreased_height()?;
                let start = self.data.len() - self.width;
                (0..self.width).for_each(|_|{ self.data.remove(start); });
                Ok(())
            },
        }
    }
    pub fn rotate(&mut self, rotation: isize) {
        let mut rotation = rotation % (self.width as isize);
        if rotation < 0 {
            rotation = rotation + self.width as isize;
        }
        let rotation = rotation as usize;
        self.data = self.data.as_slice()
            .chunks(self.width)
            .map(|arr|arr.into_iter().cycle().skip(rotation).take(self.width))
            .flatten()
            .map(Clone::clone)
            .collect()
    }
    pub fn map<X: Debug + Clone, F: Fn(&T)->X>(&self, fun: F) -> Grid<X> {
        Grid {
            width: self.width,
            height: self.height,
            data: self.data.iter().map(
                |(obj, first)|(fun(obj), *first)
            ).collect(),
        }
    }
}

impl<T: Debug + Clone + Default> Grid<T> {
    pub fn resize(&mut self, Size {width, height}: Size) {
        let width = width.get();
        let height = height.get();
        if width > self.width {
            let delta = width - self.width;
            for _ in 0..delta {
                self.grow(Side::Right, Default::default());
            }
        } else {
            let delta = self.width - width;
            for _ in 0..delta {
                self.shrink(Side::Right).unwrap();
            }
        }
        if height > self.height {
            let delta = height - self.height;
            for _ in 0..delta {
                self.grow(Side::Bottom, Default::default());
            }
        } else {
            let delta = self.height - height;
            for _ in 0..delta {
                self.shrink(Side::Bottom).unwrap();
            }
        }
    }
}

impl<T: Debug + Default + Clone> Default for Grid<T> {
    fn default() -> Self {
        Self::new(Size::default(), T::default())
    }
}

#[test]
fn negative_remainder() {
    assert_eq!(-3, -3 % 10);
    assert_eq!(-3, -13 %10);
}