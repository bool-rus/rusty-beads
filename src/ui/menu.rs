use crate::reimport::*;
use button::State;
use super::pallette::Pallette;
use crate::ui::AsContainer;
use crate::entities::{Message, Color};
use iced::{Svg, svg, Scrollable, scrollable};
use crate::grid::Grid;
use crate::beads::Beads;

#[derive(Default)]
pub struct TopMenu {
    palette: Pallette,
    grow: State,
    shrink: State,
    export: State,
    load: State,
}

impl AsContainer<Message> for TopMenu {
    fn as_container(&mut self) -> Container<'_, Message> {
        Container::new(Row::new()
            .push(Button::new(&mut self.load, Text::new("Load")).on_press(Message::OpenPressed))
            .push(Button::new(&mut self.export, Text::new("Export")).on_press(Message::ExportPressed))
            .push(Button::new(&mut self.grow, Text::new("+")).on_press(Message::GrowPressed))
            .push(Button::new(&mut self.shrink, Text::new("-")).on_press(Message::ShrinkPressed))
            .push(self.palette.as_container())
            .spacing(5))
    }
}

#[derive(Default)]
pub struct RightMenu {
    beads_btn: State,
    show_beads: bool,
    beads: Beads<Color>,
    beads_scroll: scrollable::State,
}

impl RightMenu {
    pub fn beads_pressed(&mut self) {
        self.show_beads = !self.show_beads;
    }
    pub fn update(&mut self, grid: &Grid<Color>) {
        if self.show_beads {
            self.beads = grid.into();
        }
    }
}

impl AsContainer<Message> for RightMenu {
    fn as_container(&mut self) -> Container<'_, Message> {
        let svg = Svg::new(svg::Handle::from_memory(super::icon::BEADS));
        let buttons = Column::new().width(Length::Units(30)).push(
            Button::new(&mut self.beads_btn, svg).on_press(Message::BeadsPressed)
        );
        let mut row = Row::new();
        if self.show_beads {
            row = row.push(Scrollable::new(&mut self.beads_scroll).push(self.beads.as_container()));
        }
        Container::new(row.push(buttons))
    }
}