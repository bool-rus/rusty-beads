use crate::reimport::*;
use super::AppWidget;
use super::widget::ColorBox;
use crate::grid::Grid;
use crate::entities::{Color, Schema, Coord};
use std::rc::Rc;
use std::cell::Cell;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub enum Message {
    Ignore,
    GridClicked(Coord),
    SetColor(Coord, Color),//TODO: надо бы убрать, здесь неактуально
    GridUpdated(Arc<Grid<Color>>),
    Rotate(isize),
    SetRotation(f32),
    SchemaChange,
    ZoomIn,
    ZoomOut,
}

pub struct GridPlate {
    grid: Arc<Grid<Color>>,
    mouse_hold: Rc<Cell<bool>>,
    schema: Rc<Cell<Schema>>,
    rotation: isize,
    scroll: scrollable::State,
    slider: slider::State,
    half_size: u16,
    rot_l: button::State,
    rot_r: button::State,
}

impl GridPlate {
    pub fn new(schema: Rc<Cell<Schema>>, mouse_hold: Rc<Cell<bool>>) -> Self {
        Self {
            grid: Arc::new(Grid::default()),
            mouse_hold ,
            schema,
            rotation: 0,
            half_size: 6,
            slider: Default::default(),
            scroll: Default::default(),
            rot_l: Default::default(),
            rot_r: Default::default(),
        }
    }
    fn switch_schema(&mut self) {
        use Schema::*;
        match self.schema.get() {
            FirstOffset => self.schema.set(Straight),
            SecondOffset => self.schema.set(FirstOffset),
            Straight => self.schema.set(SecondOffset),
        }
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
        let portions = match self.schema.get() {
            Schema::FirstOffset => [full, half, full],
            Schema::SecondOffset => [half, full, half],
            Schema::Straight => [half, half, half],
        };
        let grid = &self.grid;
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
                    .take(width);
                children.extend(iter.map(|(item, col)| {
                    let coord = Coord{x:row, y:col};
                    let mut widget = ColorBox::new(item.clone())
                        .width(full)
                        .height(full)
                        .on_press(Message::GridClicked(coord).into());
                    if self.mouse_hold.get() {
                        widget = widget.on_over(Message::GridClicked(coord))
                    }
                    widget.into()
                }));
                children.push(
                    Space::new(portions[index+1],full).into()
                );
                Row::with_children(children).into()
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

    fn update(&mut self, msg: Message) {
        use Message::*;
        match msg {
            GridUpdated(grid) => self.grid = grid,
            Rotate(rotation) => { self.rotation += rotation; }
            SetRotation(rotation) => {
                let width = self.grid.width() as f32;
                let rotation = width*rotation;
                self.rotation = rotation.round() as isize;
            }
            ZoomIn => { self.half_size += 1; }
            ZoomOut => if self.half_size > 1 { self.half_size -= 1; },
            SchemaChange => self.switch_schema(),
            Ignore | GridClicked(..) => {}
            SetColor(..) =>{}
        }
    }
}
