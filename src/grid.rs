
use std::num::NonZeroUsize;
#[derive(Debug)]
pub enum Error {
    InvalidDataSize,
}

pub struct Grid<T> {
    width: usize,
    height: usize,
    data: Vec<T>,
}

impl<T:Clone> Grid<T> {
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
    pub fn grow(&mut self, value: T) {
        let width = self.width + 2;
        let height = self.height + 2;
        let mut data = Vec::with_capacity(width * height);
        for _ in 0..width {
            data.push(value.clone());
        }
        self.as_table().into_iter().for_each(|row|{
            data.push(value.clone());
            data.extend_from_slice(row);
            data.push(value.clone());
        });
        for _ in 0..width {
            data.push(value.clone());
        }
        self.width = width;
        self.height = height;
        self.data = data;
    }
    pub fn shrink(&mut self) -> Result<(), String>{
        if self.height < 4 || self.width < 4 {
            return Err("Cannot shrink".to_owned())
        }
        let width = self.width - 2;
        let height = self.height - 2;
        let mut data = Vec::with_capacity(width*height);
        let table = self.as_table();
        let table = table.as_slice();
        let len = table.len();
        &table[1..len-1].iter().for_each(|row|{
            data.extend_from_slice(&row[1..row.len()-1]);
        });
        self.width = width;
        self.height = height;
        self.data = data;
        Ok(())
    }
    pub fn as_data(&self) -> &[T]{
        self.data.as_slice()
    }
}



impl<T: Default + Clone> Default for Grid<T> {
    fn default() -> Self {
        let value = NonZeroUsize::new(10usize).unwrap();
        Self::new(value, value, T::default())
    }
}