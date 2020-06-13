use crate::reimport::*;
use crate::entities;
use super::AppWidget;
use super::widget::ColorBox;
use crate::grid::Grid;
use crate::beads::Beads;
use iced::Align;
use crate::entities::Color;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, Clone, Copy)]
pub enum Message {
    GridClicked(usize,usize)
}

pub struct GridPlate {
    color: Color,
    grid: Rc<RefCell<Grid<Color>>>,
}

impl GridPlate {
    pub fn new(grid: Rc<RefCell<Grid<Color>>>) -> Self {
        Self { grid, color: Default::default() }
    }
}

impl AppWidget for GridPlate {
    type Message = Message;
    type UpdateData = Color;


    fn view(&mut self) -> Element<'_, Message> {
        let portions = [2u16,1,2];
        Container::new(Column::with_children(
            self.grid.borrow().as_table()
                .iter().enumerate().map(|(row, arr)| {
                let mut children= Vec::with_capacity(arr.len() + 2);
                let index = row % 2;
                children.push(Element::from(
                    Space::new(Length::FillPortion(portions[index]),Length::Fill)
                ));
                children.extend(arr.iter().enumerate().map(|(col,item)| {
                    ColorBox::new(item.clone())
                        .width(Length::FillPortion(2))
                        .height(Length::FillPortion(2))
                        .on_press(Message::GridClicked(row, col)
                            .into()
                        ).into()
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
            Message::GridClicked(row, col) => {
                self.grid.borrow_mut().set(row,col, self.color);
            }
        }
    }

    fn update_data(&mut self, data: &Self::UpdateData) {
        self.color = data.clone();
    }
}


impl<'a> AppWidget for Beads<entities::Color> {
    type Message = super::RightMenuMessage;
    type UpdateData = ();

    fn view(&mut self) -> Element<'_, Self::Message> {
        let col = Column::with_children(
            self.iter().map(|(color, count)|{
                Row::new().spacing(5).align_items(Align::Center)
                    .push(ColorBox::new(color.clone()))
                    .push(Text::new(count.to_string()))
                    .into()
            }).collect()
        ).spacing(1);
        Container::new(col).into()
    }
}