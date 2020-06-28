use crate::reimport::*;
use super::AppWidget;
use super::widget::ColorBox;
use crate::grid::Grid;
use crate::entities::{Color, GridAction, Side};
use std::rc::Rc;
use std::cell::{RefCell, Cell};
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy)]
pub enum Message {
    GridClicked(usize,usize),
    SetColor(usize, usize, Color),
    GridAction(GridAction),
    Rotate(isize),
    SetRotation(f32),
    SchemaChange,
    ZoomIn,
    ZoomOut,
    Undo,
    Redo,
}

pub struct GridPlate {
    grid: Rc<RefCell<Grid<Color>>>,
    mouse_hold: Rc<Cell<bool>>,
    first_offset: Rc<Cell<bool>>,
    undo: VecDeque<Message>,
    redo: VecDeque<Message>,
    rotation: isize,
    scroll: scrollable::State,
    slider: slider::State,
    half_size: u16,
    rot_l: button::State,
    rot_r: button::State,
}

impl GridPlate {
    pub fn new(grid: Rc<RefCell<Grid<Color>>>, first_offset: Rc<Cell<bool>>, mouse_hold: Rc<Cell<bool>>) -> Self {
        Self {
            grid,
            mouse_hold ,
            first_offset,
            undo: VecDeque::with_capacity(1000),
            redo: VecDeque::with_capacity(1000),
            rotation: 0,
            half_size: 5,
            slider: Default::default(),
            scroll: Default::default(),
            rot_l: Default::default(),
            rot_r: Default::default(),
        }
    }
    fn update_impl(&mut self, msg: Message, log_undo: bool) -> Result<(), String> {
        let undo = match msg {
            Message::SetColor(row, col, color) => {
                let prev_color = self.grid.borrow_mut().set(row,col, color)?;
                if prev_color != color {
                    Some(Message::SetColor(row, col, prev_color))
                } else { None }
            }
            Message::GridAction(action) => {
                match action {
                    GridAction::Add(side) => {
                        if matches!(side, Side::Top) {
                            self.first_offset.set(!self.first_offset.get());
                        }
                        self.grid.borrow_mut().grow(side, Color::default());
                        Some(Message::GridAction(GridAction::Remove(side)))
                    },
                    GridAction::Remove(side) => {
                        if matches!(side, Side::Top) {
                            self.first_offset.set(!self.first_offset.get());
                        }
                        self.grid.borrow_mut().shrink(side);
                        Some(Message::GridAction(GridAction::Add(side)))
                    },
                }
            }
            Message::Undo  => match self.undo.pop_front() {
                Some(msg) => {
                    self.update_impl(msg, false);
                    None
                }
                None => None
            }
            Message::Redo => match self.redo.pop_front() {
                Some(msg) => {
                    self.update_impl(msg, true);
                    None
                }
                None => None
            }
            Message::GridClicked(..) => {
                self.redo.clear();
                None
            },
            Message::Rotate(rotation) => {
                self.rotation += rotation;
                None
            }
            Message::SetRotation(rotation) => {
                let width = self.grid.borrow().width() as f32;
                let rotation = width*rotation;
                self.rotation = rotation.round() as isize;
                None
            }
            Message::ZoomIn => {
                self.half_size += 1;
                None
            }
            Message::ZoomOut => {
                if self.half_size > 1 {
                    self.half_size -= 1;
                }
                None
            }
            Message::SchemaChange => {
                self.first_offset.set(!self.first_offset.get());
                None
            }
        };
        let deque = if log_undo { &mut self.undo } else { &mut self.redo };
        if let Some(undo) = undo {
            deque.push_front(undo);
        }
        Ok(())
    }
}

fn normalize_rotation(rot: isize, width: usize) -> usize {
    let width = width as isize;
    let modulo = rot % width;
    if modulo >= 0 { modulo as usize} else { (width + modulo) as usize }
}

impl AppWidget for GridPlate {
    type Message = Message;


    fn view(&mut self) -> Element<'_, Message> {
        let full = Length::Units(self.half_size * 2);
        let half = Length::Units(self.half_size);
        let portions = if self.first_offset.get() { [full,half,full] } else { [half,full,half] };
        let grid = self.grid.borrow();
        let width = grid.width();
        let range = 0..width;
        let rotation = normalize_rotation(self.rotation, width);
        let grid = Column::with_children(
            grid.as_table().iter().enumerate().map(|(row, arr)| {
                let mut children= Vec::with_capacity(arr.len() + 2);
                let index = row % 2;
                children.push(Element::from(
                    Space::new(portions[index],full)
                ));
                let iter = arr.iter()
                    .cycle()
                    .zip(range.clone().into_iter().cycle())
                    .skip(rotation)
                    .zip(range.clone().into_iter());
                children.extend(iter.map(|((item, col), _)| {
                    let mut widget = ColorBox::new(item.clone())
                        .width(full)
                        .height(full)
                        .on_press(Message::GridClicked(row, col).into());
                    if self.mouse_hold.get() {
                        widget = widget.on_over(Message::GridClicked(row,col))
                    }
                    widget.into()
                    //Text::new(format!("{}",item.b)).width(Length::FillPortion(2)).into()
                }));
                children.push(
                    Space::new(portions[index+1],full).into()
                );
                Row::with_children(children)
                    //.height(Length::Fill)
                    .into()
            }).collect());
        let grid = Container::new(Scrollable::new(&mut self.scroll).push(grid))
            .width(Length::Fill)
            .height(Length::Fill)
            .align_y(Align::Center)
            .align_x(Align::Center);
        Column::new().push(grid).push(Row::new()
            .push(Container::new(
                Button::new(&mut self.rot_l, Text::new("<")).on_press(Message::Rotate(-1))
            ).width(Length::FillPortion(1)).align_x(Align::Start))
            .push(Slider::new(
                &mut self.slider,
                -1.0..=1.0,
                (self.rotation as f32)/(width as f32),
                |v|{Message::SetRotation(v)}
            ).width(Length::FillPortion(8)))
            .push(Container::new(
                Button::new(&mut self.rot_r, Text::new(">")).on_press(Message::Rotate(1))
            ).width(Length::FillPortion(1)).align_x(Align::End))
        ).into()
    }

    fn update(&mut self, msg: Self::Message) {
        self.update_impl(msg, true);
    }
}
