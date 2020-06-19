use crate::reimport::*;
use super::{AppWidget, icon, palette};
use super::RightPanelState;

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
    use crate::iced::{button, scrollable, svg, Svg, Scrollable};
    use std::rc::Rc;
    use std::cell::Cell;

    #[derive(Default)]
    pub struct RightMenu {
        beads_btn: button::State,
        beads_showed: bool,
    }


    #[derive(Debug,Clone,Copy)]
    pub enum Message {
        ShowBeads,
        Hide,
    }

    impl AppWidget for RightMenu {
        type Message = Message;
        fn view(&mut self) -> Element<'_, Message> {
            let svg = Svg::new(svg::Handle::from_memory(icon::BEADS_LINE));
            let msg = if self.beads_showed { Message::Hide } else { Message::ShowBeads };
            let buttons = Column::new().width(Length::Fill).push(
                Button::new(&mut self.beads_btn, svg).on_press(msg)
            );
            Container::new(buttons).into()
        }

        fn update(&mut self, msg: Message) {
            match msg {
                Message::ShowBeads => { self.beads_showed = true },
                Message::Hide => { self.beads_showed = false },
            }
        }
    }
}

pub mod left {
    use super::*;
    use button::State;
    use iced::{Svg, svg};

    #[derive(Debug, Copy, Clone)]
    pub enum Message {
        AddTop,
        AddLeft,
        AddRight,
        AddBottom,
        RemoveTop,
        RemoveLeft,
        RemoveRight,
        RemoveBottom,
    }

    #[derive(Default)]
    pub struct Menu {
        add_top: State,
        add_left: State,
        add_right: State,
        add_bottom: State,
        remove_top: State,
        remove_left: State,
        remove_right: State,
        remove_bottom: State,
    }

    fn svg(data: &[u8]) -> Svg {
        Svg::new(svg::Handle::from_memory(data))
    }

    impl AppWidget for Menu {
        type Message = Message;

        fn view(&mut self) -> Element<'_, Self::Message> {
            Column::new().width(Length::Fill).spacing(5)
                .push(Button::new(&mut self.add_top, svg(icon::ADD_TOP_ROW)).on_press(Message::AddTop))
                .push(Button::new(&mut self.add_left, svg(icon::ADD_LEFT_COLUMN)).on_press(Message::AddLeft))
                .push(Button::new(&mut self.add_right, svg(icon::ADD_RIGHT_COLUMN)).on_press(Message::AddRight))
                .push(Button::new(&mut self.add_bottom, svg(icon::ADD_BOTTOM_ROW)).on_press(Message::AddBottom))
                .push(Button::new(&mut self.remove_top, svg(icon::REMOVE_TOP_ROW)).on_press(Message::RemoveTop))
                .push(Button::new(&mut self.remove_left, svg(icon::REMOVE_LEFT_COLUMN)).on_press(Message::RemoveLeft))
                .push(Button::new(&mut self.remove_right, svg(icon::REMOVE_RIGHT_COLUMN)).on_press(Message::RemoveRight))
                .push(Button::new(&mut self.remove_bottom, svg(icon::REMOVE_BOTTOM_ROW)).on_press(Message::RemoveBottom))
                .into()
        }
    }

}