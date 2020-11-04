use crate::model::*;
use crate::model;
use crate::reimport::*;
use super::AppWidget;
use std::collections::HashSet;
use std::sync::Arc;
use std::cmp::Ord;

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
        let mut sorted: Vec<_> = self.model.as_ref().as_ref().colors().iter().map(|(c,&a)|{
            (c.clone(), a)
        }).collect();
        sorted.sort_unstable_by_key(|(color, ..)|{
            let Color {r, g, b} = color;
            (*r as u32) + (*g as u32) + (*b as u32)
        });
        let [top, bot] = sorted.into_iter()
            .zip(self.buttons.iter_mut())
            .map(|((color, active),btn)| {
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