use crate::reimport::*;
use super::AppWidget;
use super::widget::ColorBox;
use std::rc::Rc;
use std::cell::Cell;
use std::sync::Arc;
use crate::model::*;
use std::fmt::Debug;


#[derive(Debug, Clone)]
pub enum Message<T: Debug + Send + Sync> {
    Ignore,
    Press(Coord),
    Move(Coord),
    GridUpdated(Arc<T>),
    Rotate(isize),
    SetRotation(f32),
    ZoomIn,
    ZoomOut,
    MouseRelease,
}

pub struct GridPlate<T> {
    grid_ref: Arc<T>,
    mouse_hold: bool,
    rotation: isize,
    scroll: scrollable::State,
    slider: slider::State,
    half_size: u16,
    rot_l: button::State,
    rot_r: button::State,
}

impl<T> GridPlate<T> {
    pub fn new(grid_ref: Arc<T>) -> Self {
        Self {
            grid_ref,
            mouse_hold: false,
            rotation: 0,
            half_size: 6,
            slider: Default::default(),
            scroll: Default::default(),
            rot_l: Default::default(),
            rot_r: Default::default(),
        }
    }
}

fn normalize_rotation(rot: isize, width: usize) -> usize {
    let width = width as isize;
    let modulo = rot % width;
    if modulo >= 0 { modulo as usize} else { (width + modulo) as usize }
}

impl<T: AsRef<BeadGrid> + Debug + Send + Sync + Clone + GetSchema> AppWidget for GridPlate<T> {
    type Message = Message<T>;

    fn view(&mut self) -> Element<'_, Message<T>> {
        let full = Length::Units(self.half_size * 2);
        let half = Length::Units(self.half_size);
        let grid = self.grid_ref.as_ref().as_ref();
        let schema = self.grid_ref.get_schema();
        let portions = match schema {
            Schema::FirstOffset => [full, half, full],
            Schema::SecondOffset => [half, full, half],
            Schema::Straight => [half, half, half],
        };
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
                children.extend(iter.map(|(Bead {color, filled}, col)| {
                    let coord = Coord{x:row, y:col};
                    let mut widget = ColorBox::new(color.clone())
                        .width(full)
                        .height(full)
                        .on_press(Message::Press(coord).into());
                    if self.mouse_hold {
                        widget = widget.on_over(Message::Move(coord));
                    }
                    if *filled {
                        widget = widget.border_color(iced::Color::WHITE);
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

    fn update(&mut self, msg: Message<T>) {
        use Message::*;
        match msg {
            MouseRelease => self.mouse_hold = false,
            GridUpdated(model) => self.grid_ref = model,
            Rotate(rotation) => { self.rotation += rotation; }
            SetRotation(rotation) => {
                let width = self.grid_ref.as_ref().as_ref().width() as f32;
                let rotation = width*rotation;
                self.rotation = rotation.round() as isize;
            }
            ZoomIn => { self.half_size += 1; }
            ZoomOut => if self.half_size > 1 { self.half_size -= 1; },
            Press(..) => self.mouse_hold = true,
            Move(..) | Ignore => {}
        }
    }
}
