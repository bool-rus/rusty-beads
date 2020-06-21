use crate::reimport::*;
use super::{AppWidget, icon, palette};
use super::RightPanelState;

pub mod top {
    use super::*;
    use palette::Palette;
    use button::State;
    use crate::entities::{GridAction, Side};
    use iced::{Svg, svg};

    #[derive(Default)]
    pub struct TopMenu {
        palette: Palette,
        export: State,
        load: State,
        undo: State,
        redo: State,
        add_left: State,
        remove_left: State,
        remove_right: State,
        add_right: State,
    }
    #[derive(Debug, Copy, Clone)]
    pub enum Message {
        OpenPressed,
        ExportPressed,
        Palette(palette::Message),
        GridAction(GridAction),
        Undo,
        Redo,
    }

    impl AppWidget for TopMenu {
        type Message = Message;

        fn view(&mut self) -> Element<'_, Message> {
            Container::new(Row::new()
                .push(Button::new(&mut self.load, Text::new("Load")).on_press(Message::OpenPressed.into()))
                .push(Button::new(&mut self.export, Text::new("Export")).on_press(Message::ExportPressed.into()))
                .push(
                    Button::new(
                        &mut self.undo,
                        Svg::new(svg::Handle::from_memory(icon::UNDO))
                    ).on_press(Message::Undo)
                )
                .push(
                    Button::new(
                        &mut self.redo,
                        Svg::new(svg::Handle::from_memory(icon::REDO))
                    ).on_press(Message::Redo)
                )
                .push(Button::new(
                    &mut self.add_left,
                    Svg::new(svg::Handle::from_memory(icon::ADD_LEFT_COLUMN)))
                    .on_press(Message::GridAction(GridAction::Add(Side::Left)))
                )
                .push(Button::new(
                    &mut self.remove_left,
                    Svg::new(svg::Handle::from_memory(icon::REMOVE_LEFT_COLUMN)))
                    .on_press(Message::GridAction(GridAction::Remove(Side::Left)))
                ).push(Button::new(
                    &mut self.remove_right,
                    Svg::new(svg::Handle::from_memory(icon::REMOVE_RIGHT_COLUMN)))
                    .on_press(Message::GridAction(GridAction::Remove(Side::Right)))
                ).push(Button::new(
                    &mut self.add_right,
                    Svg::new(svg::Handle::from_memory(icon::ADD_RIGHT_COLUMN)))
                    .on_press(Message::GridAction(GridAction::Add(Side::Right)))
                )
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
    use crate::entities::{GridAction, Side};

    #[derive(Debug, Copy, Clone)]
    pub enum Message {
        ToggleResize,
        GridAction(GridAction),
        ZoomIn,
        ZoomOut,
    }

    #[derive(Default)]
    pub struct Menu {
        toggle_resize: State,
        zoom_in: State,
        zoom_out: State,
        add_top: State,
        add_bottom: State,
        remove_top: State,
        remove_bottom: State,
    }

    fn svg(data: &[u8]) -> Svg {
        Svg::new(svg::Handle::from_memory(data))
    }

    impl AppWidget for Menu {
        type Message = Message;

        fn view(&mut self) -> Element<'_, Self::Message> {
            let add_top = Message::GridAction(GridAction::Add(Side::Top));
            let add_bottom = Message::GridAction(GridAction::Add(Side::Bottom));
            let remove_top = Message::GridAction(GridAction::Remove(Side::Top));
            let remove_bottom = Message::GridAction(GridAction::Remove(Side::Bottom));

            Column::new().width(Length::Fill).spacing(5)
                .push(Button::new(&mut self.toggle_resize, svg(icon::RESIZE_ICON)).on_press(Message::ToggleResize))
                .push(Button::new(&mut self.zoom_in, svg(icon::ZOOM_IN)).on_press(Message::ZoomIn))
                .push(Button::new(&mut self.zoom_out, svg(icon::ZOOM_OUT)).on_press(Message::ZoomOut))
                .push(Button::new(&mut self.add_top, svg(icon::ADD_TOP_ROW)).on_press(add_top))
                .push(Button::new(&mut self.remove_top, svg(icon::REMOVE_TOP_ROW)).on_press(remove_top))
                .push(Button::new(&mut self.remove_bottom, svg(icon::REMOVE_BOTTOM_ROW)).on_press(remove_bottom))
                .push(Button::new(&mut self.add_bottom, svg(icon::ADD_BOTTOM_ROW)).on_press(add_bottom))
                .into()
        }
    }

}