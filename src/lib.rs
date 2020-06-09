
use std::fmt;
use std::str::{FromStr};
use std::num::ParseIntError;


#[derive(Clone, Hash, Copy, Debug, Eq, PartialEq)]
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


#[derive(Debug, Clone, Copy)]
pub enum Message {
    OpenPressed,
    ExportPressed,
    GrowPressed,
    ShrinkPressed,
    PlateClicked(usize, usize),
    SetColor(Color)
}
