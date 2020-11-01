use crate::model::*;
use crate::model;
use crate::reimport::*;
use super::AppWidget;
use std::collections::HashSet;
use std::sync::Arc;

pub trait AsPalette :  std::fmt::Debug + AsRef<model::Palette<Color>> {}
impl <T: AsRef<model::Palette<Color>>  + std::fmt::Debug > AsPalette for T {}

type PaletteArc = Arc<dyn AsPalette + Send + Sync>;



pub struct Palette {
    model: PaletteArc,
    buttons: Vec<button::State>,
}

#[derive(Debug, Clone)]
pub enum Message {
    ActivateColor(Color),
    Updated(PaletteArc),
}

fn create_buttons(palette: &model::Palette<Color>) -> Vec<button::State> {
    (1..palette.colors().len())
        .map(|_|button::State::new())
        .collect()
}

impl Palette {
    pub fn new(model: PaletteArc) -> Self {
        let buttons = create_buttons(model.as_ref().as_ref());
        Self { model, buttons }
    }
}

impl AppWidget for Palette {
    type Message = Message;

    fn view(&mut self) -> Element<'_, Message> {
        let [top, bot] = self.model.as_ref().as_ref().colors().iter()
            .zip(self.buttons.iter_mut())
            .map(|((color, &active),btn)| {
                let space = Space::new(Length::Units(7), Length::Units(5));
                let mut button = Button::new(
                    btn,
                    space,
                ).style(crate::ui::style::ColorButton(color.clone().into()));
                if !active {
                    button = button.on_press(Message::ActivateColor(color.clone()));
                }
                button
            }).enumerate()
            .fold([Vec::new(), Vec::new()], |mut vecs, (i, btn)|{
                vecs[i%2].push(Element::new(btn));
                vecs
            });
        Column::new()
            .push(Row::with_children(top))
            .push(Row::with_children(bot))
            .into()
    }

    fn update(&mut self, msg: Self::Message) {
        match msg {
            Message::Updated(model) => {
                self.buttons = create_buttons(model.as_ref().as_ref());
                self.model = model;
            },
            _ => {}
        }
    }
}
/*
impl Default for Palette {
    fn default() -> Self {
        Palette::new(vec![
            Color { r: 0x61 ,   g: 0x00,    b: 0x00 },
            Color { r: 0xff,    g: 0x00,    b: 0x88 },
            Color { r: 0x98,    g: 0x02,    b: 0x2f },
            Color { r: 0xcf,    g: 0x32,    b: 0x00 },
            Color { r: 0xff,    g: 0x32,    b: 0x32 },
            Color { r: 0xfd,    g: 0x8c,    b: 0x0e },
            Color { r: 0xff,    g: 0xe6,    b: 0x11 },
            Color { r: 0xff,    g: 0xfc,    b: 0x72 },
            Color { r: 0x88,    g: 0x0c,    b: 0x14 },
            Color { r: 0xb0,    g: 0x5e,    b: 0x07 },
            Color { r: 0x29,    g: 0x13,    b: 0x9c },
            Color { r: 0x3f,    g: 0x9b,    b: 0xe3 },
            Color { r: 0x69,    g: 0xc7,    b: 0xac },
            Color { r: 0x9a,    g: 0xcc,    b: 0xb0 },
            Color { r: 0x49,    g: 0x8c,    b: 0x55 },
            Color { r: 0x00,    g: 0xb1,    b: 0x5a },
            Color { r: 0x3e,    g: 0xe0,    b: 0x19 },
            Color { r: 0x8d,    g: 0xe4,    b: 0x6f },
            Color { r: 0x8c,    g: 0x62,    b: 0xd3 },
            Color { r: 0xc8,    g: 0xb5,    b: 0xff },
            Color { r: 0xff,    g: 0xff,    b: 0xff },
            Color { r: 0xdd,    g: 0xdd,    b: 0xdd },
            Color { r: 0x90,    g: 0x93,    b: 0x9e },
            Color { r: 0x00,    g: 0x00,    b: 0x00 },
        ])
    }
}
 */