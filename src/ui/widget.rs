use iced_native::{Widget, layout, Layout, MouseCursor, Event, Clipboard};
use iced_wgpu::{Primitive, Renderer, Defaults};
use iced_native::input::{mouse, ButtonState};
use iced::{Size, Color, Element, Length, Point, Background};
use crate::wrapper::Wrappable;
use std::hash::Hash;
use std::cell::Cell;
use std::rc::Rc;

pub struct ColorBox<T> {
    color: Color,
    width: Length,
    height: Length,
    press_message: Option<T>,
    over_message: Option<T>,
}

impl <T> ColorBox<T> {
    pub fn new<C: Into<Color>>(color: C) -> Self {
        Self {
            color: color.into(),
            width: Length::Units(30),
            height: Length::Units(30),
            press_message: None,
            over_message: None,
        }
    }
    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }
    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }
    pub fn on_press(mut self, msg: T) -> Self {
        self.press_message = Some(msg);
        self
    }
    pub fn on_over(mut self, msg: T) -> Self {
        self.over_message = Some(msg);
        self
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
        if let Some(ref msg) = self.press_message {
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
        if let (Some(msg), Event::Mouse(mouse::Event::CursorMoved {..})) = (&self.over_message, event) {
            if layout.bounds().contains(cursor_position) {
                messages.push(msg.clone())
            }
        };
    }
}

impl<'a, M: 'a + Clone> Into<Element<'a,M>> for ColorBox<M> {
    fn into(self) -> Element<'a,M> {
        Element::new(self)
    }
}

pub struct MouseListener(Rc<Cell<bool>>);

impl MouseListener {
    pub fn new(hold: Rc<Cell<bool>>) -> Self {
        Self(hold)
    }
}

impl<Message: Clone> Widget<Message, Renderer> for MouseListener {
    fn width(&self) -> Length {
        Length::Units(0)
    }

    fn height(&self) -> Length {
        Length::Units(0)
    }

    fn layout(&self, _renderer: &Renderer, _limits: &layout::Limits) -> layout::Node {
        layout::Node::new(Size::ZERO)
    }

    fn draw(
        &self,
        _renderer: &mut Renderer,
        _defaults: &Defaults,
        _layout: Layout<'_>,
        _cursor_position: Point,
    ) -> (Primitive, MouseCursor) {
        (Primitive::None, MouseCursor::OutOfBounds)
    }

    fn hash_layout(&self, state: &mut iced_native::Hasher) {
        0.hash(state)
    }

    fn on_event(&mut self,
                event: Event,
                _layout: Layout<'_>,
                _cursor_position: Point,
                _messages: &mut Vec<Message>,
                _renderer: &Renderer,
                _clipboard: Option<&dyn Clipboard>) {
        if let Event::Mouse(mouse::Event::Input{state, button: mouse::Button::Left}) = event {
            match state {
                ButtonState::Pressed => {self.0.set(true)},
                ButtonState::Released => {self.0.set(false)},
            }
        }
    }
}