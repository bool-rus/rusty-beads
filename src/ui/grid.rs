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
    Undo,
    Redo,
}

pub struct GridPlate {
    grid: Rc<RefCell<Grid<Color>>>,
    mouse_hold: Rc<Cell<bool>>,
    first_offset: Rc<Cell<bool>>,
    undo: VecDeque<Message>,
    redo: VecDeque<Message>,
    btn_undo: button::State,
    btn_redo: button::State,
}

impl GridPlate {
    pub fn new(grid: Rc<RefCell<Grid<Color>>>, first_offset: Rc<Cell<bool>>, mouse_hold: Rc<Cell<bool>>) -> Self {
        Self {
            grid,
            mouse_hold ,
            first_offset,
            undo: VecDeque::with_capacity(1000),
            redo: VecDeque::with_capacity(1000),
            btn_undo: Default::default(),
            btn_redo: Default::default(),
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
            _ => {None}
        };
        let deque = if log_undo { &mut self.undo } else { &mut self.redo };
        if let Some(undo) = undo {
            deque.push_front(undo);
        }
        Ok(())
    }
}

impl AppWidget for GridPlate {
    type Message = Message;


    fn view(&mut self) -> Element<'_, Message> {
        let portions = if self.first_offset.get() { [2u16,1,2] } else { [1u16,2,1] };
        Container::new(Column::with_children(
            self.grid.borrow().as_table()
                .iter().enumerate().map(|(row, arr)| {
                let mut children= Vec::with_capacity(arr.len() + 2);
                let index = row % 2;
                children.push(Element::from(
                    Space::new(Length::FillPortion(portions[index]),Length::Fill)
                ));
                children.extend(arr.iter().enumerate().map(|(col,item)| {
                    let mut widget = ColorBox::new(item.clone())
                        .width(Length::FillPortion(2))
                        .height(Length::FillPortion(2))
                        .on_press(Message::GridClicked(row, col).into());
                    if self.mouse_hold.get() {
                        widget = widget.on_over(Message::GridClicked(row,col))
                    }
                    widget.into()
                    //Text::new(format!("{}",item.b)).width(Length::FillPortion(2)).into()
                }));
                children.push(
                    Space::new(Length::FillPortion(portions[index+1]),Length::Fill).into()
                );
                Row::with_children(children)
                    .height(Length::Fill)
                    .into()
            }).collect())
            .push(Button::new(&mut self.btn_undo, Text::new("undo")).on_press(Message::Undo))
            .push(Button::new(&mut self.btn_redo, Text::new("redo")).on_press(Message::Redo))
        ).into()
    }

    fn update(&mut self, msg: Self::Message) {
        self.update_impl(msg, true);
    }
}
