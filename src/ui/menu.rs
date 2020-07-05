use crate::reimport::*;
use super::{AppWidget, icon, palette};
use super::RightPanelState;
use super::style::ToggledOn;

pub mod top {
    use super::*;
    use palette::Palette;
    use button::State;
    use crate::entities::{GridAction, Side};

    #[derive(Default)]
    pub struct TopMenu {
        palette: Palette,
        save: State,
        load: State,
        undo: State,
        redo: State,
        add_left: State,
        remove_left: State,
        remove_right: State,
        add_right: State,
        active_mode: ActiveMode,
    }

    #[derive(Debug,Clone,Copy)]
    enum ActiveMode {
        Empty,
        Save,
        Open,
    }

    impl Default for ActiveMode {
        fn default() -> Self {
            Self::Empty
        }
    }

    #[derive(Debug, Copy, Clone)]
    pub enum Message {
        Ignore,
        Hide,
        Open,
        Save,
        Palette(palette::Message),
        GridAction(GridAction),
        Undo,
        Redo,
    }

    impl AppWidget for TopMenu {
        type Message = Message;

        fn view(&mut self) -> Element<'_, Message> {
            let mut btn_load = Button::new(&mut self.load, icon::OPEN.svg()).on_press(Message::Open);
            let mut btn_save = Button::new(&mut self.save, icon::SAVE.svg()).on_press(Message::Save);
            match self.active_mode {
                ActiveMode::Empty => {},
                ActiveMode::Save => {btn_save = btn_save.on_press(Message::Hide).style(ToggledOn)},
                ActiveMode::Open => {btn_load = btn_load.on_press(Message::Hide).style(ToggledOn)},
            }
            Container::new(Row::new()
                .push(btn_load)
                .push(btn_save)
                .push(
                    Button::new(
                        &mut self.undo,
                        icon::UNDO.svg()
                    ).on_press(Message::Undo)
                )
                .push(
                    Button::new(
                        &mut self.redo,
                        icon::REDO.svg()
                    ).on_press(Message::Redo)
                )
                .push(Button::new(
                    &mut self.add_left,
                    icon::ADD_LEFT_COLUMN.svg())
                    .on_press(Message::GridAction(GridAction::Add(Side::Left)))
                )
                .push(Button::new(
                    &mut self.remove_left,
                    icon::REMOVE_LEFT_COLUMN.svg())
                    .on_press(Message::GridAction(GridAction::Remove(Side::Left)))
                ).push(Button::new(
                    &mut self.remove_right,
                    icon::REMOVE_RIGHT_COLUMN.svg())
                    .on_press(Message::GridAction(GridAction::Remove(Side::Right)))
                ).push(Button::new(
                    &mut self.add_right,
                    icon::ADD_RIGHT_COLUMN.svg())
                    .on_press(Message::GridAction(GridAction::Add(Side::Right)))
                )
                .push(self.palette.view().map(From::from))
                .spacing(5)).into()
        }

        fn update(&mut self, msg: Self::Message) {
            match msg {
                Message::Hide => { self.active_mode = ActiveMode::Empty },
                Message::Open => { self.active_mode = ActiveMode::Open },
                Message::Save => { self.active_mode = ActiveMode::Save },
                _ => {}
            }
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
    use iced::{button, scrollable, Scrollable};
    use std::rc::Rc;
    use std::cell::Cell;

    #[derive(Default)]
    pub struct RightMenu {
        beads_btn: button::State,
        beads_showed: bool,
    }


    #[derive(Debug,Clone,Copy)]
    pub enum Message {
        Ignore,
        ShowBeads,
        Hide,
    }

    impl AppWidget for RightMenu {
        type Message = Message;
        fn view(&mut self) -> Element<'_, Message> {
            let mut beads_btn = Button::new(&mut self.beads_btn, icon::BEADS_LINE.svg())
                .on_press(Message::ShowBeads);
            if self.beads_showed {
                beads_btn = beads_btn.on_press(Message::Hide).style(ToggledOn);
            };
            let buttons = Column::new().width(Length::Fill).push(beads_btn );
            Container::new(buttons).into()
        }

        fn update(&mut self, msg: Message) {
            match msg {
                Message::ShowBeads => { self.beads_showed = true },
                Message::Hide => { self.beads_showed = false },
                Message::Ignore => {}
            }
        }
    }
}

pub mod left {
    use super::*;
    use button::State;
    use crate::entities::{GridAction, Side};

    #[derive(Debug, Copy, Clone)]
    pub enum Message {
        Ignore,
        ShowResize,
        Hide,
        GridAction(GridAction),
        SchemaChange,
        ZoomIn,
        ZoomOut,
    }

    #[derive(PartialEq, Clone, Copy)]
    enum ActiveMode {
        Empty,
        Resize,
    }

    impl Default for ActiveMode {
        fn default() -> Self {
            ActiveMode::Empty
        }
    }

    #[derive(Default)]
    pub struct Menu {
        active: ActiveMode,
        toggle_resize: State,
        zoom_in: State,
        zoom_out: State,
        schema_change: State,
        add_top: State,
        add_bottom: State,
        remove_top: State,
        remove_bottom: State,
    }

    impl AppWidget for Menu {
        type Message = Message;

        fn view(&mut self) -> Element<'_, Self::Message> {
            let add_top = Message::GridAction(GridAction::Add(Side::Top));
            let add_bottom = Message::GridAction(GridAction::Add(Side::Bottom));
            let remove_top = Message::GridAction(GridAction::Remove(Side::Top));
            let remove_bottom = Message::GridAction(GridAction::Remove(Side::Bottom));
            let mut resize_btn = Button::new(&mut self.toggle_resize, icon::RESIZE.svg()).on_press(Message::ShowResize);

            match self.active {
                ActiveMode::Empty => {},
                ActiveMode::Resize => { resize_btn = resize_btn.on_press(Message::Hide).style(ToggledOn) },
            }

            Column::new().width(Length::Fill).spacing(5)
                .push(resize_btn)
                .push(Button::new(&mut self.zoom_in, icon::ZOOM_IN.svg()).on_press(Message::ZoomIn))
                .push(Button::new(&mut self.zoom_out, icon::ZOOM_OUT.svg()).on_press(Message::ZoomOut))
                .push(Button::new(&mut self.schema_change, icon::CHANGE_SCHEMA.svg()).on_press(Message::SchemaChange))
                .push(Button::new(&mut self.add_top, icon::ADD_TOP_ROW.svg()).on_press(add_top))
                .push(Button::new(&mut self.remove_top, icon::REMOVE_TOP_ROW.svg()).on_press(remove_top))
                .push(Button::new(&mut self.remove_bottom, icon::REMOVE_BOTTOM_ROW.svg()).on_press(remove_bottom))
                .push(Button::new(&mut self.add_bottom, icon::ADD_BOTTOM_ROW.svg()).on_press(add_bottom))
                .into()
        }

        fn update(&mut self, msg: Self::Message) {
            match msg {
                Message::ShowResize => {
                    self.active = ActiveMode::Resize;
                },
                Message::Hide => {
                    self.active = ActiveMode::Empty;
                }
                _ => {}
            }
        }
    }

}