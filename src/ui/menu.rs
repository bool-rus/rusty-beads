use crate::reimport::*;
use button::State;
use super::pallette::Pallette;
use crate::ui::AsContainer;
use crate::entities::{StandartMessage, Color};
use iced::{Svg, svg, Scrollable, scrollable};
use crate::grid::Grid;
use crate::beads::Beads;
use super::icon;

#[derive(Default)]
pub struct TopMenu {
    palette: Pallette,
    grow: State,
    shrink: State,
    export: State,
    load: State,
}

impl<M: Clone + From<StandartMessage> + 'static> AsContainer<M> for TopMenu {
    fn as_container(&mut self) -> Container<'_, M> {
        Container::new(Row::new()
            .push(Button::new(&mut self.load, Text::new("Load")).on_press(StandartMessage::OpenPressed.into()))
            .push(Button::new(&mut self.export, Text::new("Export")).on_press(StandartMessage::ExportPressed.into()))
            .push(Button::new(&mut self.grow, Text::new("+")).on_press(StandartMessage::GrowPressed.into()))
            .push(Button::new(&mut self.shrink, Text::new("-")).on_press(StandartMessage::ShrinkPressed.into()))
            .push(self.palette.as_container())
            .spacing(5))
    }
}

pub mod right {
    use crate::reimport::*;
    use crate::entities::Color;
    use crate::beads::Beads;
    use crate::{Grid, AsContainer};
    use crate::iced::{button,scrollable, svg, Svg};

    #[derive(Default)]
    pub struct RightMenu {
        beads_btn: button::State,
        show_beads: bool,
        beads: Beads<Color>,
        beads_scroll: scrollable::State,
    }

    impl RightMenu {
        pub fn beads_pressed(&mut self) {
            self.show_beads = !self.show_beads;
        }
        pub fn update_grid(&mut self, grid: &Grid<Color>) {
            if self.show_beads {
                self.beads = grid.into();
            }
        }
        pub fn update(&mut self, msg: Message) {
            match msg {
                Message::BeadsPressed => { self.show_beads = !self.show_beads }
            }
        }
    }

    #[derive(Debug,Clone,Copy)]
    pub enum Message {
        BeadsPressed,
    }

    impl<M: Clone + From<Message> + 'static> AsContainer<M> for RightMenu {
        fn as_container(&mut self) -> Container<'_, M> {
            let svg = Svg::new(svg::Handle::from_memory(super::icon::BEADS));
            let buttons = Column::new().width(Length::Units(30)).push(
                Button::new(&mut self.beads_btn, svg).on_press(Message::BeadsPressed.into())
            );
            let mut row = Row::new();
            if self.show_beads {
                row = row.push(Scrollable::new(&mut self.beads_scroll).push(self.beads.as_container()));
            }
            Container::new(row.push(buttons))
        }
    }
}