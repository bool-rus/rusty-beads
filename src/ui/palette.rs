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
        let mut rows = [Vec::new(), Vec::new()];
        self.buttons.iter_mut().enumerate().for_each(|(i, (color, state))|{
            let index = i % 2;
            rows[index].push(Button::new(
                state,
                Space::new(Length::Units(7), Length::Units(7)),
            ).on_press(Message::SetColor(color.clone()))
                .style(crate::ui::style::ColorButton(color.clone().into()))
                .into());
        });
        let [top, bot] = rows;
        Column::new()
            .push(Row::with_children(top))
            .push(Row::with_children(bot))
            .into()
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