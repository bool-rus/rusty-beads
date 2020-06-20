use crate::reimport::*;
use super::AppWidget;
use super::widget::ColorBox;
use crate::grid::Grid;
use crate::entities::{Color, GridAction, Side};
use std::rc::Rc;
use std::cell::{RefCell, Cell};

#[derive(Debug, Clone, Copy)]
pub enum Message {
    GridClicked(usize,usize),
    SetColor(usize, usize, Color),
    GridAction(GridAction)
}

pub struct GridPlate {
    grid: Rc<RefCell<Grid<Color>>>,
    mouse_hold: Rc<Cell<bool>>,
    first_offset: Rc<Cell<bool>>,
}

impl GridPlate {
    pub fn new(grid: Rc<RefCell<Grid<Color>>>, first_offset: Rc<Cell<bool>>, mouse_hold: Rc<Cell<bool>>) -> Self {
        Self { grid, mouse_hold , first_offset }
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
            }).collect())).into()
    }

    fn update(&mut self, msg: Self::Message) {
        match msg {
            Message::SetColor(row, col, color) => {
                self.grid.borrow_mut().set(row,col, color);
            }
            Message::GridAction(action) => {
                match action {
                    GridAction::Add(side) => {
                        if matches!(side, Side::Top) {
                            self.first_offset.set(!self.first_offset.get());
                        }
                        self.grid.borrow_mut().grow(side, Color::default());
                    },
                    GridAction::Remove(side) => {
                        if matches!(side, Side::Top) {
                            self.first_offset.set(!self.first_offset.get());
                        }
                        self.grid.borrow_mut().shrink(side);
                    },
                }
            }
            _ => {/*doing nothing*/}
        }
    }
}
