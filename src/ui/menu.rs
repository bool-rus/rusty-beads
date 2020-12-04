use crate::reimport::*;
use super::{AppWidget, icon, palette};
use super::style::ToggledOn;
use super::SvgButton;
use crate::model::{Color, Model};
use std::sync::Arc;

pub mod top {
    use super::*;
    use super::palette::Palette;

    pub struct TopMenu {
        palette: Palette,
        save: SvgButton,
        load: SvgButton,
        undo: SvgButton,
        redo: SvgButton,

        active_mode: ActiveMode,
    }

    impl TopMenu {
        pub fn new(model: Arc<Model<Color>>) -> Self {
            TopMenu {
                palette: Palette::new(model),
                save: SvgButton::new(icon::SAVE),
                load: SvgButton::new(icon::OPEN),
                undo: SvgButton::new(icon::UNDO),
                redo: SvgButton::new(icon::REDO),
                active_mode: Default::default(),
            }
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

    #[derive(Debug, Clone)]
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
    use iced::button;

    enum Activated {
        Beads,
        Colors,
        None,
    }

    impl Default for Activated {
        fn default() -> Self {
            Activated::None
        }
    }

    #[derive(Default)]
    pub struct RightMenu {
        beads_btn: button::State,
        colors_btn: button::State,
        activated: Activated,
    }


    #[derive(Debug,Clone,Copy)]
    pub enum Message {
        Ignore,
        ShowBeads,
        ShowColors,
        Hide,
    }

    impl AppWidget for RightMenu {
        type Message = Message;
        fn view(&mut self) -> Element<'_, Message> {
            let mut beads_btn = Button::new(&mut self.beads_btn, icon::BEADS_LINE.svg())
                .on_press(Message::ShowBeads);
            let mut colors_btn = Button::new(&mut self.colors_btn, icon::CONFIG_COLOR.svg())
                .on_press(Message::ShowColors);
            use Activated::*;
            match self.activated {
                Beads => beads_btn = beads_btn.on_press(Message::Hide).style(ToggledOn),
                Colors => colors_btn = colors_btn.on_press(Message::Hide).style(ToggledOn),
                None => {},
            }
            let buttons = Column::new().width(Length::Fill).push(beads_btn ).push(colors_btn);
            Container::new(buttons).into()
        }

        fn update(&mut self, msg: Message) {
            match msg {
                Message::ShowBeads => self.activated = Activated::Beads,
                Message::ShowColors => self.activated = Activated::Colors,
                Message::Hide => self.activated = Activated::None,
                Message::Ignore => {}
            }
        }
    }
}

pub mod left {
    use super::*;

    #[derive(Debug, Copy, Clone)]
    pub enum Message {
        Ignore,
        ShowResize,
        Hide,
        SchemaChange,
        ZoomIn,
        ZoomOut,
        MoveSeam(isize),
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

    pub struct Menu { //TODO: зарефакторить все под SvgButton
        active: ActiveMode,
        toggle_resize: SvgButton,
        zoom_in: SvgButton,
        zoom_out: SvgButton,
        schema_change: SvgButton,
        seam_left: SvgButton,
        seam_right: SvgButton,
    }
    
    impl Default for Menu {
        fn default() -> Self {
            use icon::*;
            Menu {
                active: Default::default(),
                toggle_resize: SvgButton::new(RESIZE),
                zoom_in: SvgButton::new(ZOOM_IN),
                zoom_out: SvgButton::new(ZOOM_OUT),
                schema_change: SvgButton::new(CHANGE_SCHEMA),
                seam_left: SvgButton::new(SEAM_LEFT),
                seam_right: SvgButton::new(SEAM_RIGHT),
            }
        }
    }

    impl AppWidget for Menu {
        type Message = Message;

        fn view(&mut self) -> Element<'_, Self::Message> {
            let (msg, toggled_on)  = match self.active {
                ActiveMode::Empty => (Message::ShowResize, false),
                ActiveMode::Resize => (Message::Hide, true),
            };
            let mut resize_btn = self.toggle_resize.button().on_press(msg);
            if toggled_on {
                resize_btn = resize_btn.style(ToggledOn);
            }
            Column::new().width(Length::Fill).spacing(5)
                .push(resize_btn)
                .push(self.zoom_in.button().on_press(Message::ZoomIn))
                .push(self.zoom_out.button().on_press(Message::ZoomOut))
                .push(self.schema_change.button().on_press(Message::SchemaChange))
                .push(self.seam_left.button().on_press(Message::MoveSeam(-1)))
                .push(self.seam_right.button().on_press(Message::MoveSeam(1)))
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