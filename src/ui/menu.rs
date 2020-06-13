use crate::reimport::*;
use super::{AppWidget, icon, palette};

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

    impl AppWidget for TopMenu {
        type Message = Message;
        type UpdateData = ();

        fn view(&mut self) -> Element<'_, Message> {
            Container::new(Row::new()
                .push(Button::new(&mut self.load, Text::new("Load")).on_press(Message::OpenPressed.into()))
                .push(Button::new(&mut self.export, Text::new("Export")).on_press(Message::ExportPressed.into()))
                .push(Button::new(&mut self.grow, Text::new("+")).on_press(Message::GrowPressed.into()))
                .push(Button::new(&mut self.shrink, Text::new("-")).on_press(Message::ShrinkPressed.into()))
                .push(self.palette.view().map(From::from))
                .spacing(5)).into()
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
    use crate::GridPlate;
    use crate::iced::{button, scrollable, svg, Svg, Scrollable};

    #[derive(Default)]
    pub struct RightMenu {
        beads_btn: button::State,
        show_beads: bool,
    }

    #[derive(Debug,Clone,Copy)]
    pub enum Message {
        BeadsPressed,
    }

    impl RightMenu {
        pub fn show_beads(&self) -> bool {
            self.show_beads
        }
    }
    impl AppWidget for RightMenu {
        type Message = Message;
        type UpdateData = ();
        fn view(&mut self) -> Element<'_, Message> {
            let svg = Svg::new(svg::Handle::from_memory(icon::BEADS));
            let buttons = Column::new().width(Length::Fill).push(
                Button::new(&mut self.beads_btn, svg).on_press(Message::BeadsPressed)
            );
            Container::new(buttons).into()
        }

        fn update(&mut self, msg: Message) {
            match msg {
                Message::BeadsPressed => { self.show_beads = !self.show_beads }
            }
        }
    }
}