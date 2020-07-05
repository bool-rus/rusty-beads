use crate::reimport::*;
use button::Style;
use iced::{Color, Background};

pub struct ColorButton(pub Color);

impl button::StyleSheet for ColorButton {
    fn active(&self) -> Style {
        Style {
            shadow_offset: Default::default(),
            background: Some(Background::Color(self.0.clone())),
            border_radius: 0,
            border_width: 0,
            border_color: Color::TRANSPARENT,
            text_color: Color::BLACK,
        }
    }
}

pub struct FSMenuItem;

pub struct ToggledOn;

impl button::StyleSheet for ToggledOn {
    fn active(&self) -> Style {
        Style {
            shadow_offset: Default::default(),
            background: Some(Background::Color(Color{
                r: 0.5,
                g: 0.5,
                b: 0.5,
                a: 1.0
            })),
            border_radius: 0,
            border_width: 0,
            border_color: Color::BLACK,
            text_color: Color::BLACK,
        }
    }
}

impl button::StyleSheet for FSMenuItem {
    fn active(&self) -> Style {
        Style {
            shadow_offset: Default::default(),
            background: None,
            border_radius: 0,
            border_width: 0,
            border_color: Color::BLACK,
            text_color: Color::BLACK,
        }
    }

    fn hovered(&self) -> Style {
        Style {
            shadow_offset: Default::default(),
            background: Some(Background::Color(Color::BLACK)),
            border_radius: 0,
            border_width: 0,
            border_color: Color::WHITE,
            text_color: Color::WHITE
        }
    }
}