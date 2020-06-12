use crate::reimport::*;
use super::{AsContainer, icon, palette};

pub mod top {
    use super::*;
    use palette::Palette;
    use button::State;

    #[derive(Default)]
    pub struct TopMenu {
        palette: Palette,
        grow: State,
        shrink: State,
        export: State,
        load: State,
    }
    #[derive(Debug, Copy, Clone)]
    pub enum Message {
        OpenPressed,
        ExportPressed,
        GrowPressed,
        ShrinkPressed,
        Palette(palette::Message),
    }

    impl<M: Clone + From<Message> + From<palette::Message> + 'static> AsContainer<M> for TopMenu {
        fn as_container(&mut self) -> Container<'_, M> {
            Container::new(Row::new()
                .push(Button::new(&mut self.load, Text::new("Load")).on_press(Message::OpenPressed.into()))
                .push(Button::new(&mut self.export, Text::new("Export")).on_press(Message::ExportPressed.into()))
                .push(Button::new(&mut self.grow, Text::new("+")).on_press(Message::GrowPressed.into()))
                .push(Button::new(&mut self.shrink, Text::new("-")).on_press(Message::ShrinkPressed.into()))
                .push(self.palette.as_container())
                .spacing(5))
        }
    }

    impl From<palette::Message> for Message {
        fn from(m: palette::Message) -> Self {
            Message::Palette(m)
        }
    }
}
pub mod right {
    use super::*;
    use crate::entities::Color;
    use crate::beads::Beads;
    use crate::Grid;
    use crate::iced::{button, scrollable, svg, Svg, Scrollable};

    #[derive(Default)]
    pub struct RightMenu {
        beads_btn: button::State,
        show_beads: bool,
        beads: Beads<Color>,
        beads_scroll: scrollable::State,
    }

    impl RightMenu {
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
            let svg = Svg::new(svg::Handle::from_memory(icon::BEADS));
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