
use std::num::NonZeroUsize;
use crate::entities::Side;
use std::fmt::Debug;

#[derive(Debug)]
pub enum Error {
    InvalidDataSize,
}

#[derive(Debug, Clone)]
pub struct Grid<T: Debug + Clone> {
    width: usize,
    height: usize,
    data: Vec<T>,
}

impl<T: Debug + Clone> Grid<T> {
    pub fn new(width: NonZeroUsize, height: NonZeroUsize, item: T) -> Self {
        let width = width.get();
        let height = height.get();
        Self {
            width,
            height,
            data: vec![item; width*height],
        }
    }
    pub fn frow_raw(width: NonZeroUsize, height: NonZeroUsize, data: Vec<T>) -> Result<Self, Error>{
        let width = width.get();
        let height = height.get();
        if data.len() != width * height {
            Err(Error::InvalidDataSize)
        } else {
            Ok(Self { width, height, data })
        }
    }
    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.height
    }
    pub fn update_from_another(&mut self, rhs: Self) {
        let Self {width,height,data} = rhs;
        self.width = width;
        self.height = height;
        self.data = data;
    }
    pub fn set(&mut self, row: usize, column: usize, value: T) -> Result<T,String> {
        let prev = self.data
            .as_mut_slice()
            .chunks_mut(self.width)
            .nth(row)
            .ok_or("row out of bounds")?
            .get_mut(column)
            .ok_or("column out of bounds")?;
        let result = prev.clone();
        *prev = value;
        Ok(result)
    }
    pub fn as_table(&self) -> Vec<&[T]> {
        self.data.as_slice().chunks(self.width).collect()
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
                    Side::Left => self.as_table().into_iter().for_each(|row| {
                        data.push(value.clone());
                        data.extend_from_slice(row);
                    }),
                    Side::Right => self.as_table().into_iter().for_each(|row|{
                        data.extend_from_slice(row);
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
                data.extend(self.as_table().iter().map(|row| {
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
}

fn lr_side(i: usize) -> Side {
    if i % 2 == 0 { Side::Right } else { Side::Left }
}

fn tb_side(i: usize) -> Side {
    if i % 2 == 0 { Side::Bottom } else { Side::Top }
}

impl<T: Debug + Clone + Default> Grid<T> {
    pub fn resize(&mut self, width: NonZeroUsize, height: NonZeroUsize) {
        let width = width.get();
        let height = height.get();
        if width > self.width {
            let delta = width - self.width;
            for i in 0..delta {
                self.grow(Side::Right, Default::default());
            }
        } else {
            let delta = self.width - width;
            for i in 0..delta {
                self.shrink(Side::Right);
            }
        }
        if height > self.height {
            let delta = height - self.height;
            for i in 0..delta {
                self.grow(Side::Bottom, Default::default());//top grow corrupts grid
            }
        } else {
            let delta = self.height - height;
            for i in 0..delta {
                self.shrink(Side::Bottom);//top shrink corrupts grid
            }
        }
    }
}



impl<T: Debug + Default + Clone> Default for Grid<T> {
    fn default() -> Self {
        let value = NonZeroUsize::new(33usize).unwrap();
        Self::new(value, value, T::default())
    }
}