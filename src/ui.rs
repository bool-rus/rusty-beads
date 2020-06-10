extern crate iced_native;
extern crate iced_wgpu;
use crate::reimport::*;
use crate::iced::{Color, Size};
use iced_native::{Widget, layout, Layout, MouseCursor, Event, Clipboard};
use iced_wgpu::{Primitive, Renderer, Defaults};
use std::hash::Hash;
use iced_native::input::{mouse, ButtonState};
use crate::wrapper::Wrappable;
use crate::lib::Message;
use crate::field::Grid;
use crate::beads::Beads;
use crate::lib;
use iced::Align;

struct ColorBox<T> {
    color: Color,
    width: Length,
    height: Length,
    message: Option<T>,
}

impl <T> ColorBox<T> {
    fn new<C: Into<Color>>(color: C, message: T) -> Self {
        Self {
            color: color.into(),
            width: Length::FillPortion(2),
            height: Length::FillPortion(2),
            message: Some(message)
        }
    }
    fn just_color<C: Into<Color>>(color: C) -> Self {
        Self {
            color: color.into(),
            width: Length::Units(30),
            height: Length::Units(30),
            message: None
        }
    }
}
impl<Message:Clone> Widget<Message, Renderer> for ColorBox<Message> {
    fn width(&self) -> Length {
        self.width
    }

    fn height(&self) -> Length {
        self.height
    }

    fn layout(
        &self,
        _renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let limits = limits.width(self.width).height(self.height);

        layout::Node::new(limits.resolve(Size::ZERO))
    }

    fn draw(
        &self,
        _renderer: &mut Renderer,
        _defaults: &Defaults,
        layout: Layout<'_>,
        _cursor_position: Point,
    ) -> (Primitive, MouseCursor) {
        (
            Primitive::Quad {
                bounds: layout.bounds(),
                background: Background::Color(self.color.clone().into()),
                border_radius: 0,
                border_width: 1,
                border_color: iced::Color::BLACK,
            },
            MouseCursor::OutOfBounds,
        )
    }

    fn hash_layout(&self, state: &mut iced_native::Hasher) {
        self.color.wrap().hash(state);
    }

    fn on_event(
        &mut self,
        event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        messages: &mut Vec<Message>,
        _renderer: &Renderer,
        _clipboard: Option<&dyn Clipboard>,
    ) {
        if let Some(ref msg) = self.message {
            match event {
                Event::Mouse(mouse::Event::Input {
                                 button: mouse::Button::Left,
                                 state: ButtonState::Pressed,
                             }) => {
                    if layout.bounds().contains(cursor_position) {
                        messages.push(msg.clone())
                    }
                },
                _ => {}
            }
        };
    }
}

impl<'a, M: 'a + Clone> Into<Element<'a,M>> for ColorBox<M> {
    fn into(self) -> Element<'a,M> {
        Element::new(self)
    }
}

pub trait AsContainer {
    fn as_container(&mut self) -> Container<'_, Message>;
}

impl AsContainer for Grid<crate::lib::Color> {
    fn as_container(&mut self) -> Container<'_, Message> {
        let portions = [2u16,1,2];
        Container::new(Column::with_children(
            self.as_table()
                .iter().enumerate().map(|(row, arr)| {
                let mut children= Vec::with_capacity(arr.len() + 2);
                let index = row % 2;
                children.push(Element::from(
                    Space::new(Length::FillPortion(portions[index]),Length::Fill)
                ));
                children.extend(arr.iter().enumerate().map(|(col,item)| {
                    ColorBox::new(item.clone(), Message::PlateClicked(row, col)).into()
                    //Text::new(format!("{}",item.b)).width(Length::FillPortion(2)).into()
                }));
                children.push(
                    Space::new(Length::FillPortion(portions[index+1]),Length::Fill).into()
                );
                Row::with_children(children)
                    .height(Length::Fill)
                    .into()
            }).collect()))
    }
}

impl AsContainer for Beads<lib::Color> {
    fn as_container(&mut self) -> Container<'_, Message> {
        let col = Column::with_children(
            self.iter().map(|(color, count)|{
                Row::new().spacing(5).align_items(Align::Center)
                    .push(ColorBox::just_color(color.clone()))
                    .push(Text::new(count.to_string()))
                    .into()
            }).collect()
        ).spacing(1);
        Container::new(col)
    }
}