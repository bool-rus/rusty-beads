use crate::reimport::*;
use super::AppWidget;
use super::widget::ColorBox;
use std::{sync::Arc, iter};
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
    box_size: u16,
    rot_l: button::State,
    rot_r: button::State,
}

impl<T> GridPlate<T> {
    pub fn new(grid_ref: Arc<T>) -> Self {
        Self {
            grid_ref,
            mouse_hold: false,
            rotation: 0,
            box_size: 15,
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

impl<T: Debug + Send + Sync + Clone + GetSchema + AsRef<BeadsLine<Bead<Color>>>> AppWidget for GridPlate<T> {
    type Message = Message<T>;

    fn view(&mut self) -> Element<'_, Message<T>> {
        let grid = self.grid_ref.as_ref().as_ref();
        let chunks = self.grid_ref.get_schema().base();
        let chunk_width = {
            let x = self.box_size as usize / chunks;
            if x == 0 { 1 } else { x }
        } as u16;
        let box_size = chunk_width * chunks as u16;
        let full = Length::Units(box_size);
        let width = grid.width();
        let rotation = normalize_rotation(self.rotation, width);
        let grid = Column::with_children(
            grid.table(rotation).map(|row|{
                let beads::BeadsRow {row, offset, iter} = row;
                let space = Space::new(Length::Units(chunk_width * offset as u16), full).into();
                let cells = iter.take(width).map(|(col, Bead {color, filled})| {
                    let coord = Coord{x:row, y:col};

                    let mut widget = ColorBox::new(color.clone())
                        .width(full)
                        .height(full)
                        .on_press(Message::Press(coord).into());
                    if self.mouse_hold {
                        widget = widget.on_over(Message::Move(coord));
                    }
                    if col == 0 {
                        widget = widget.border_color(iced::Color::from_rgb(0.9, 0.0, 0.0))
                    }
                    if *filled {
                        widget = widget.border_color(iced::Color::WHITE);
                    }
                    widget.into()
                });
                Row::with_children(iter::once(space).chain(cells).collect()).into()
            }).collect()
        );
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
            ZoomIn => { self.box_size += 1; }
            ZoomOut => if self.box_size > 1 { self.box_size -= 1; },
            Press(..) => self.mouse_hold = true,
            Move(..) | Ignore => {}
        }
    }
}
