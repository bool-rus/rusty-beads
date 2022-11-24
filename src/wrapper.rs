use std::hash::{Hasher, Hash};

use egui::Color32;

pub struct Wrapped<T>(T);
pub trait Wrappable where Self: Sized{
    fn wrap(&self)->Wrapped<&Self>;
    fn own(self) -> Wrapped<Self>;
}
impl<T> Wrappable for T {
    fn wrap(&self) -> Wrapped<&Self> {
        Wrapped(self)
    }
    fn own(self) -> Wrapped<Self> {
        Wrapped(self)
    }
}

pub struct Compressed<I, T>{
    item: Option<(T, usize)>,
    iter: I,
}

impl<I, T> Iterator for Compressed<I, T> where I: Iterator<Item = T>, T: Eq {
    type Item = (T, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let mut current = None;
        std::mem::swap(&mut current, &mut self.item);
        let mut current = current?;
        while let Some(next) = self.iter.next() {
            if next == current.0 {
                current.1 += 1
            } else {
                self.item = Some((next, 1));
                return Some(current);
            }
        }
        Some(current)
    }
}

pub trait Compressable<I,T> {
    fn compress(self) -> Compressed<I,T>;
}

impl<I,T> Compressable<I,T> for I where I: Iterator<Item=T>, T: Clone {
    fn compress(mut self) -> Compressed<I,T> {
        let item = self.next().map(|x|(x, 1));
        Compressed { item, iter: self }
    }
}

pub struct Uncompressed<'a,I,T> {
    item: &'a T,
    left: usize,
    iter: I,
}
impl<'a,I,T> Clone for Uncompressed<'a,I,T> where I: Clone {
    fn clone(&self) -> Self {
        Self { item: self.item, left: self.left, iter: self.iter.clone() }
    }
}

impl<'a, I, T> Iterator for Uncompressed<'a,I,T> where I: Iterator<Item=&'a (T, usize)>, T: 'a {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.left > 0 {
            self.left -= 1;
            Some(self.item)
        } else {
            let (item, left) = self.iter.next()?;
            self.item = item;
            self.left = *left;
            self.next()
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (lower, higher) = self.iter.size_hint();
        (lower + self.left, higher.map(|x|x + self.left))
    }
}

pub trait Uncompressable<'a,I,T> {
    fn uncompress(self) -> Uncompressed<'a,I,T>;
}

impl<'a,I,T> Uncompressable<'a,I,T> for I where I: Iterator<Item=&'a (T,usize)>, T: 'a {
    fn uncompress(mut self) -> Uncompressed<'a,I,T> {
        let (item, left) = self.next().unwrap();
        Uncompressed { item, left: *left, iter: self}
    }
}

pub struct Chunked<I> {
    chunk_size: usize,
    iter: I,
}

impl<'a,I,T> Iterator for Chunked<I> where I: Iterator<Item = &'a T> + 'a, T: 'a{
    type Item = Vec<&'a T>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buf = Vec::with_capacity(self.chunk_size);
        for _ in 0..self.chunk_size {
            buf.push(self.iter.next()?);
        }
        Some(buf)  
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (lower, higher) = self.iter.size_hint();
        (lower/self.chunk_size + 1, higher.map(|n|n/self.chunk_size + 1))
    }
}

pub trait Chunkable {
    fn chunks(self, chunk_size: usize) -> Chunked<Self> where Self: Sized;
}

impl<'a,I,T> Chunkable for I where I: Iterator<Item = &'a T> + 'a, T: 'a {
    fn chunks(self, chunk_size: usize) -> Chunked<Self> where Self: Sized {
        Chunked { chunk_size, iter: self }
    }
}

pub trait Invertable {
    fn invert(&self) -> Self;
}

impl Invertable for Color32 {
    fn invert(&self) -> Self {
        let [r,g,b, _] = self.to_array();
        let (r,g,b) = (r as f32 / 255.0, g as f32 / 255.0, b as f32 /255.0);
        let light = g + (r + b) / 2.0;
        if light > 0.6 {
            Color32::BLACK
        } else {
            Color32::WHITE
        }
    }
}

#[cfg(test)] 
mod test {

    use super::*;
    
    #[test]
    fn test_compress() {
        let x = vec![1,2,2,3,3,3,4,4,4,4,5,5,5,5,5];
        let compressed: Vec<_> = x.into_iter().compress().collect();
        assert_eq!(
            vec![(1,1), (2,2), (3,3), (4,4), (5,5)],
            compressed
        );
        
        let x = "aabbbbccccc";
        let z: Vec<_> = x.as_bytes().into_iter().map(|x|*x).compress().collect();
        let y: Vec<_> = x.as_bytes().into_iter().compress().map(|(i,c)|(*i,c)).collect();
        assert_eq!(
            vec![(b'a',2), (b'b',4), (b'c',5)],
            z
        );
        assert_eq!(z,y);
    }

    #[test]
    fn test_uncompress() {
        let x: Vec<_> = vec![(1,1), (2,2), (3,3), (4,4), (5,5)].as_slice().into_iter().uncompress().copied().collect();
        assert_eq!(
            vec![1,2,2,3,3,3,4,4,4,4,5,5,5,5,5],
            x
        );
        let x: Vec<_> = vec![(b'a',2), (b'b',4), (b'c',5)].as_slice().into_iter().uncompress().copied().collect();
        assert_eq!(
            "aabbbbccccc".as_bytes(),
            x.as_slice()
        )
    }


    #[test]
    fn test_cycle() {
        let x = [(b'a',3),(b'x',2)];
        let unc: Vec<_> = x.iter().uncompress().cycle().take(10).copied().collect();
        assert_eq!(
            b"aaaxxaaaxx",
            unc.as_slice()
        )
    }

}