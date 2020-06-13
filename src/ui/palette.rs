use crate::entities::Color;
use crate::reimport::*;
use crate::ui::AppWidget;

pub struct Palette {
    buttons: Vec<(Color, button::State)>,
}

#[derive(Debug, Copy, Clone)]
pub enum Message {
    SetColor(Color),
}

impl Palette {
    pub fn new(colors: Vec<Color>) -> Self {
        Self {
            buttons: colors.into_iter().map(|item| { (item, Default::default()) }).collect(),
        }
    }
    pub fn add(&mut self, color: Color) {
        self.buttons.push((color, Default::default()));
    }
}

impl AppWidget for Palette {
    type Message = Message;

    fn view(&mut self) -> Element<'_, Message> {
        Container::new(Row::with_children(
            self.buttons.iter_mut().map(|(color, state)| {
                Button::new(
                    state,
                    Space::new(Length::Units(20), Length::Units(20)),
                )
                    .on_press(
                        Message::SetColor(color.clone())
                    )
                    .style(crate::ui::style::ColorButton(color.clone().into()))
                    .into()
            }).collect()
        )).into()
    }
}

impl Default for Palette {
    fn default() -> Self {
        Palette::new(
            vec![
                Color {r: 0, g:0, b:0},
                Color {r: 255, g:255, b:255},
                Color {r: 200, g:0, b:0},
                Color {r: 0, g:200, b:0},
                Color {r: 0, g:0, b:200},
            ])
    }
}