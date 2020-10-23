
use std::fmt;
use std::str::{FromStr};
use std::num::{ParseIntError, NonZeroUsize};
use serde::{Serialize, Deserialize};
use std::fmt::Debug;
use std::hash::Hash;


#[derive(Clone, Hash, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Default for Color {
    fn default() -> Self {
        Self {r:255,g:255,b:255}
    }
}

impl fmt::UpperHex for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }
}

impl fmt::LowerHex for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }
}

#[derive(Debug)]
pub enum ParseColorError {
    WrongLen,
    WrongFirstSymbol,
    Parse(ParseIntError),
    Encoding,
}

impl From<ParseIntError> for ParseColorError {
    fn from(e: ParseIntError) -> Self {
        ParseColorError::Parse(e)
    }
}

impl FromStr for Color {
    type Err = ParseColorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 7 {
            Err(ParseColorError::WrongLen)
        } else if !s.is_ascii() {
            Err(ParseColorError::Encoding)
        } else if !s.starts_with('#') {
            Err(ParseColorError::WrongFirstSymbol)
        } else {
            Ok(Self {
                r: u8::from_str_radix(&s[1..3], 16)?,
                g: u8::from_str_radix(&s[3..5], 16)?,
                b: u8::from_str_radix(&s[5..7], 16)?,
            })
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:X}", self)
    }
}

impl Into<iced::Color> for Color {
    fn into(self) -> iced::Color {
        let Self {r,g,b} = self;
        iced::Color::from_rgb8(r,g,b)
    }
}

impl From<iced::Color> for Color {
    fn from(color: iced::Color) -> Self {
        let max = u8::MAX as f32;
        Self {
            r: (max * color.r) as u8,
            g: (max * color.g) as u8,
            b: (max * color.b) as u8,
        }
    }
}
#[derive(Debug, Copy, Clone)]
pub enum Side { Top, Left, Right, Bottom }

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum Schema {
    FirstOffset,
    SecondOffset,
    Straight,
}

impl Schema {
    pub fn switch(self) -> Self {
        use Schema::*;
        match self {
            FirstOffset => SecondOffset,
            SecondOffset => Straight,
            Straight => FirstOffset,
        }
    }
}

impl Default for Schema {
    fn default() -> Self {
        Schema::FirstOffset
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Size {
    pub width: NonZeroUsize,
    pub height: NonZeroUsize,
}

impl Default for Size {
    fn default() -> Self {
        Self {
            width: NonZeroUsize::new(33).unwrap(),
            height: NonZeroUsize::new(33).unwrap(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
}


pub trait ColorTrait: Debug + Clone + Hash + Eq + PartialEq {}

impl<T> ColorTrait for T where T: Debug + Clone + Hash + Eq + PartialEq {}

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
