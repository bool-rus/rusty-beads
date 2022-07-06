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
    size: Size,
    data: Vec<(T, bool)>,
}

#[derive(Debug, Clone)]
pub struct SimplifiedGrid<T: Debug + Clone> {
    size: Size,
    data: Vec<T>,
}

impl <T: Debug + Clone + Default> SimplifiedGrid<T> {
    pub fn from_raw(width: NonZeroUsize, data: Vec<T>) -> Self {
        let height = NonZeroUsize::new(data.len()/width.get()).unwrap();
        let size = Size {width, height: height};
        Self {size, data}
    }
    pub fn size(&self) -> Size {
        self.size
    }
    pub fn resize(&mut self, Size {width, height}: Size) {
        let width = width.get();
        let height = height.get();
        if width > self.size.width() {
            let delta = width - self.size.width();
            for _ in 0..delta {
                self.grow(Side::Right, Default::default());
            }
        } else {
            let delta = self.size.width() - width;
            for _ in 0..delta {
                self.shrink(Side::Right).unwrap();
            }
        }
        if height > self.size.height() {
            let delta = height - self.size.height();
            for _ in 0..delta {
                self.grow(Side::Bottom, Default::default());
            }
        } else {
            let delta = self.size.height() - height;
            for _ in 0..delta {
                self.shrink(Side::Bottom).unwrap();
            }
        }
    }

    pub fn as_table_iter(&self) -> impl Iterator<Item=impl DoubleEndedIterator<Item=&T> + Clone> {
        self.data.as_slice()
        .chunks(self.size.width())
        .map(IntoIterator::into_iter)
    }

    pub fn grow(&mut self, side: Side, value: T) {
        match side {
            Side::Top => {
                self.size.height = self.size.height.increase();
                let mut data = Vec::with_capacity(self.size.capacity());
                for _ in 0..self.size.width() {
                    data.push(value.clone())
                }
                data.extend_from_slice(self.data.as_slice());
                self.data = data;
            },
            Side::Left | Side::Right => {
                let newsize = Size {
                    height: self.size.height,
                    width: self.size.width.increase(),
                };
                let mut data = Vec::with_capacity(newsize.capacity());
                match side {
                    Side::Left => self.as_table_iter().for_each(|row| {
                        data.push(value.clone());
                        data.extend(row.map(Clone::clone));
                    }),
                    Side::Right => self.as_table_iter().for_each(|row|{
                        data.extend(row.map(Clone::clone));
                        data.push(value.clone());
                    }),
                    _ => {unreachable!()},
                };
                self.size = newsize;
                self.data = data;
            },
            Side::Bottom => {
                self.size.height = self.size.height.increase();
                self.data.reserve_exact(self.size.width());
                for _ in 0..self.size.width() {
                    self.data.push(value.clone());
                }
            }
        }
    }
    pub fn shrink(&mut self, side: Side) -> Result<(), String> {
        match side {
            Side::Top => {
                self.size.height = self.size.height.decrease().ok_or("cannot decrease height")?;
                self.data = self.data.iter().skip(self.size.width.get()).map(Clone::clone).collect();
                Ok(())
            },
            Side::Left | Side::Right => {
                let newsize = Size {
                    height: self.size.height,
                    width: self.size.width.decrease().ok_or("cannot decrease width")?,
                };
                let range = match side {
                    Side::Left => 1..self.size.width(),
                    Side::Right => 0..newsize.width(),
                    _ => unreachable!(),
                };
                let mut newdata = Vec::with_capacity(newsize.capacity());
                newdata.extend(
                    self.data.chunks(self.size.width())
                    .map(|row| 
                        (&row[range.clone()]).iter().map(Clone::clone)
                    ).flatten()
                );
                self.size = newsize;
                self.data = newdata;
                Ok(())
            },
            Side::Bottom => {
                self.size.height = self.size.height.decrease().ok_or("cannot decrease height")?;
                let width = self.size.width.get();
                let start = self.data.len() - width;
                (0..width).for_each(|_|{ self.data.remove(start); });
                Ok(())
            },
        }
    }
    pub fn rotate(&mut self, rotation: isize) {
        let mut rotation = rotation % (self.size.width() as isize);
        if rotation < 0 {
            rotation = rotation + self.size.width() as isize;
        }
        let rotation = rotation as usize;
        self.data = self.data.as_slice()
            .chunks(self.size.width())
            .map(|arr|arr.into_iter().cycle().skip(rotation).take(self.size.width()))
            .flatten()
            .map(Clone::clone)
            .collect()
    }
}


impl<T: Debug + Clone> Grid<T> {
    pub fn new(size: Size, item: T) -> Self {
        Self {
            size,
            data: vec![(item, false); size.capacity()], //TODO: возможно, стоит переделать
        }
    }
    pub fn frow_raw(width: NonZeroUsize, data: Vec<(T, bool)>) -> Result<Self, Error> {
        if data.len() % width.get() > 0 {
            Err(Error::InvalidDataSize)
        } else {
            let height = NonZeroUsize::new(data.len()/width.get()).ok_or(Error::InvalidDataSize)?;
            Ok(Self { size: Size{width, height}, data })
        }
    }
    pub fn map<X: Debug + Clone, F: Fn(&T)->X>(&self, fun: F) -> Grid<X> {
        Grid {
            size: self.size,
            data: self.data.iter().map(
                |(obj, first)|(fun(obj), *first)
            ).collect(),
        }
    }
    pub fn simplify(&self) -> SimplifiedGrid<T> {
        SimplifiedGrid {
            size: self.size,
            data: self.data.iter().map(|(obj,_)|obj.clone()).collect()
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