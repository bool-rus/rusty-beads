use crate::entities::Color;
use crate::reimport::*;
use crate::ui::AppWidget;
use std::collections::HashSet;

pub struct Palette {
    buttons: Vec<(Color, button::State)>,
    active_color: usize,
}

#[derive(Debug, Clone)]
pub enum Message {
    ActivateColor(usize),
    AddColor(Color),
    RemoveColor,
    Loaded(HashSet<Color>),
}

impl Palette {
    pub fn new(colors: Vec<Color>) -> Self {
        Self {
            buttons: colors.into_iter().map(|item| { (item, Default::default()) }).collect(),
            active_color: 0,
        }
    }
    fn add(&mut self, color: Color) {
        self.buttons.push((color, Default::default()));
    }

    fn remove(&mut self) {
        if self.buttons.len() > 1 {
            self.buttons.remove(self.active_color);
            self.active_color = std::cmp::min(self.active_color, self.buttons.len() - 1)
        }

    }
    pub fn active_color(&self) -> Color {
        self.buttons.get(self.active_color).unwrap().0 //TODO:  обработать none
    }
}

impl AppWidget for Palette {
    type Message = Message;

    fn view(&mut self) -> Element<'_, Message> {
        let mut rows = [Vec::new(), Vec::new()];
        let active_color = self.active_color;
        self.buttons.iter_mut().enumerate().for_each(|(i, (color, state))|{
            let index = i % 2;
            let space = Space::new(Length::Units(7), Length::Units(5));
            let mut button = Button::new(
                state,
                space,
            ).style(crate::ui::style::ColorButton(color.clone().into()));
            if active_color != i {
                button = button.on_press(Message::ActivateColor(i))
            }
            rows[index].push(button.into());
        });
        let [top, bot] = rows;
        Column::new()
            .push(Row::with_children(top))
            .push(Row::with_children(bot))
            .into()
    }

    fn update(&mut self, msg: Self::Message) {
        match msg {
            Message::ActivateColor(i) => self.active_color = i,
            Message::AddColor(color) => self.add(color),
            Message::RemoveColor => self.remove(),
            Message::Loaded(colors) => self.buttons = colors.into_iter()
                .map(|color|{(color, Default::default())})
                .collect(),
        }
    }
}

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