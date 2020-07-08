use crate::reimport::*;
use super::{AppWidget, icon, palette};
use super::RightPanelState;
use super::style::ToggledOn;
use super::SvgButton;

pub mod top {
    use super::*;
    use palette::Palette;
    use button::State;
    use crate::entities::{GridAction, Side};

    pub struct TopMenu {
        palette: Palette,
        save: SvgButton,
        load: SvgButton,
        undo: SvgButton,
        redo: SvgButton,

        active_mode: ActiveMode,
    }

    impl Default for TopMenu {
        fn default() -> Self {
            TopMenu {
                palette: Default::default(),
                save: SvgButton::new(icon::SAVE),
                load: SvgButton::new(icon::OPEN),
                undo: SvgButton::new(icon::UNDO),
                redo: SvgButton::new(icon::REDO),
                active_mode: Default::default(),
            }
        }
    }

    impl TopMenu {
        pub fn palette(&self) -> &Palette {
            &self.palette
        }
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
        Undo,
        Redo,
    }

    impl AppWidget for TopMenu {
        type Message = Message;

        fn view(&mut self) -> Element<'_, Message> {
            let mut btn_load = self.load.button().on_press(Message::Open);
            let mut btn_save = self.save.button().on_press(Message::Save);
            match self.active_mode {
                ActiveMode::Empty => {},
                ActiveMode::Save => {btn_save = btn_save.on_press(Message::Hide).style(ToggledOn)},
                ActiveMode::Open => {btn_load = btn_load.on_press(Message::Hide).style(ToggledOn)},
            }
            Container::new(Row::new()
                .push(btn_load)
                .push(btn_save)
                .push(
                    self.undo.button().on_press(Message::Undo)
                )
                .push(
                    self.redo.button().on_press(Message::Redo)
                )
                .push(self.palette.view().map(From::from))
                .spacing(5)).into()
        }

        fn update(&mut self, msg: Self::Message) {
            match msg {
                Message::Palette(msg) => self.palette.update(msg),
                Message::Hide => self.active_mode = ActiveMode::Empty,
                Message::Open => self.active_mode = ActiveMode::Open,
                Message::Save => self.active_mode = ActiveMode::Save,
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
    }

    impl AppWidget for Menu {
        type Message = Message;

        fn view(&mut self) -> Element<'_, Self::Message> {
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