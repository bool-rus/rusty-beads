use crate::reimport::*;
use crate::model::*;
use super::{files, icon, style, widget};
use super::{AppWidget, SvgButton};

pub mod left {
    use super::*;
    use std::num::{ParseIntError, NonZeroUsize};
    use super::files::Message as FilesMessage;
    use super::files::FSMenu;
    use crate::io::default_dir;

    #[derive(Debug, Clone)]
    pub enum Message {
        Ignore,
        ShowResize,
        ShowOpen,
        ShowSave,
        Hide,
        Resize(Size),
        InputWidth(String),
        InputHeight(String),
        Grow(Side),
        Shrink(Side),
        FS(FilesMessage),
    }

    impl From<FilesMessage> for Message {
        fn from(msg: FilesMessage) -> Self {
            Message::FS(msg)
        }
    }

    pub enum State {
        Empty,
        Resize(ResizeWidget),
        FS(Box<dyn AppWidget<Message=FilesMessage>>),
    }

    pub struct Panel {
        size: Size,
        state: State,
    }

    impl Default for Panel {
        fn default() -> Self {
            Self {
                size: Size::default(),
                state: State::Empty,
            }
        }
    }

    impl AppWidget for Panel {
        type Message = Message;

        fn view(&mut self) -> Element<'_, Self::Message> {
            match self.state {
                State::Empty => {Space::new(Length::Units(0), Length::Units(0)).into()},
                State::Resize(ref mut widget) => { widget.view().into() },
                State::FS(ref mut files) => {files.view().map(From::from)},
            }
        }

        fn update(&mut self, msg: Self::Message) {
            use Message::*;
            match msg {
                Hide => { self.state = State::Empty },
                ShowResize => { self.state = State::Resize(ResizeWidget::new(self.size))},
                ShowOpen => { self.state = State::FS(Box::new(FSMenu::open(default_dir())))},
                ShowSave => { self.state = State::FS(Box::new(FSMenu::save(default_dir())))},
                Resize(size) => {
                    self.size = size;
                    if matches!(self.state, State::Resize(_)) {
                        self.state = State::Resize(ResizeWidget::new(self.size));
                    }
                }
                msg => {
                    match self.state {
                        State::Empty => {},
                        State::Resize(ref mut widget) => {widget.update(msg)},
                        State::FS(ref mut widget) => {
                            match msg {
                                Message::FS(FilesMessage::Open(..)) | Message::FS(FilesMessage::Save(..)) => self.state = State::Empty,
                                Message::FS(msg) => {widget.update(msg)},
                                _ => {}
                            }
                        }
                    }
                }
            }
        }
    }
    pub struct ResizeWidget {
        input_width: text_input::State,
        input_height: text_input::State,
        width: String,
        height: String,
        btn_resize: button::State,
        grow_shirnk_buttons: GrowShrinkButtons,
    }
    impl ResizeWidget {
        fn new(size: Size) -> Self {
            Self {
                input_width: Default::default(),
                input_height: Default::default(),
                width: size.width.to_string(),
                height: size.height.to_string(),
                btn_resize: Default::default(),
                grow_shirnk_buttons: Default::default(),
            }
        }
     }

    struct GrowShrinkButtons {
        add_top: SvgButton,
        add_left: SvgButton,
        add_right: SvgButton,
        add_bottom: SvgButton,
        remove_top: SvgButton,
        remove_left: SvgButton,
        remove_right: SvgButton,
        remove_bottom: SvgButton,
    }

    impl Default for GrowShrinkButtons {
        fn default() -> Self {
            use super::icon::*;
            GrowShrinkButtons {
                add_top: SvgButton::new(ADD_TOP_ROW),
                add_left: SvgButton::new(ADD_LEFT_COLUMN),
                add_right: SvgButton::new(ADD_RIGHT_COLUMN),
                add_bottom: SvgButton::new(ADD_BOTTOM_ROW),
                remove_top: SvgButton::new(REMOVE_TOP_ROW),
                remove_left: SvgButton::new(REMOVE_LEFT_COLUMN),
                remove_right: SvgButton::new(REMOVE_RIGHT_COLUMN),
                remove_bottom: SvgButton::new(REMOVE_BOTTOM_ROW),
            }
        }
    }

    impl GrowShrinkButtons {
        fn grow(side: Side) -> Message {
            Message::Grow(side)
        }
        fn shrink(side: Side) -> Message {
            Message::Shrink(side)
        }
        fn view(&mut self) -> Element<'_, Message> {
            use Side::*;
            Column::new()
                .push(
                    Row::new().height(Length::Units(30))
                        .push(space())
                        .push(self.add_top.button().on_press(Self::grow(Top)))
                )
                .push(
                    Row::new().height(Length::Units(30))
                        .push(self.add_left.button().on_press(Self::grow(Left)))
                        .push(space()).push(self.add_right.button().on_press(Self::grow(Right)))
                )
                .push(
                    Row::new().height(Length::Units(30))
                        .push(space())
                        .push(self.add_bottom.button().on_press(Self::grow(Bottom)))
                )
                .push(space())
                .push(
                    Row::new().height(Length::Units(30))
                        .push(space())
                        .push(self.remove_top.button().on_press(Self::shrink(Top)))
                )
                .push(
                    Row::new().height(Length::Units(30))
                        .push(self.remove_left.button().on_press(Self::shrink(Left)))
                        .push(space())
                        .push(self.remove_right.button().on_press(Self::shrink(Right)))
                )
                .push(
                    Row::new().height(Length::Units(30))
                        .push(space())
                        .push(self.remove_bottom.button().on_press(Self::shrink(Bottom)))
                )
                .into()
        }
    }

    fn resize_message(width: &str, height: &str) -> Result<Message, ParseIntError> {
        let width = NonZeroUsize::new(width.parse()?).unwrap();
        let height = NonZeroUsize::new(height.parse()?).unwrap();
        Ok(Message::Resize(Size {width, height}))
    }

    impl AppWidget for ResizeWidget {
        type Message = Message;
        fn view(&mut self) -> Element<'_, Self::Message> {
            let width_field = TextInput::new(
                &mut self.input_width,
                &"10",
                &self.width,
                |s| Message::InputWidth(s),
            );
            let height_field = TextInput::new(
                &mut self.input_height,
                &"10",
                &self.height,
                |s| Message::InputHeight(s),
            );

            let mut btn_ok = Button::new(&mut self.btn_resize, Text::new("OK"));
            if let Ok(msg) = resize_message(&self.width, &self.height) {
                btn_ok = btn_ok.on_press(msg);
            }
            let edit = Row::new()
                .push(Column::new()
                    .push(Text::new("Width: "))
                    .push(Text::new("Height: "))
                ).push(Column::new().width(Length::Units(50))
                .push(width_field)
                .push(height_field)
                .push(btn_ok)
            );
            Column::new().align_items(Align::Center)
                .push(edit).push(space()).push(self.grow_shirnk_buttons.view()).into()
        }

        fn update(&mut self, msg: Self::Message) {
            match msg {
                Message::Resize(_) => {/* top level process */},
                Message::InputWidth(s) => { self.width = s },
                Message::InputHeight(s) => { self.height = s },
                _ => {}
            }
        }
    }
    fn space() -> Space {
        Space::new(Length::Units(30), Length::Units(30))
    }
}

pub mod right {
    use super::*;
    use super::widget::{ColorBox, Gradient};
    use std::collections::HashMap;
    use std::sync::Arc;
    use super::style::Colored;
    use super::icon;
    use std::fmt::Debug;

    #[derive(Debug, Copy, Clone)]
    pub enum ColorPart {
        Hue(f32),
        Saturation(f32),
        Lightness(f32),
    }
    #[derive(Debug, Clone)]
    pub enum Message {
        Ignore,
        ShowBeads,
        ShowColors,
        Hide,
        GridUpdated(Arc<dyn AsBeadsLine + Send + Sync>),
        ToggleCheckbox(usize),
        AddColor(Color),
        ConfigColor(ColorPart),
        RemoveColor,
    }

    #[derive(Debug)]
    enum State {
        None,
        Beads(BeadsWidget),
        Colors(ColorMenu),
    }

    pub struct RightPanel {
        line_ref: Arc<dyn AsBeadsLine>,
        scroll: scrollable::State,
        state: State,
    }

    impl RightPanel {
        pub fn new(line_ref: Arc<dyn AsBeadsLine>) -> Self {
            Self {
                line_ref,
                scroll: Default::default(),
                state: State::None,
            }
        }
    }

    impl AppWidget for RightPanel {
        type Message = Message;

        fn view(&mut self) -> Element<'_, Self::Message> {
            Scrollable::new(&mut self.scroll).push(
                match self.state {
                    State::None => { Space::new(Length::Units(0), Length::Units(0)).into() }
                    State::Beads(ref mut widget) => { widget.view() }
                    State::Colors(ref mut widget) => widget.view()
                })
                .into()
        }

        fn update(&mut self, msg: Self::Message) {
            match &mut self.state {
                State::Beads(ref mut widget) => widget.update(msg.clone()),
                State::Colors(ref mut widget) => widget.update(msg.clone()),
                State::None => {},
            }
            match msg {
                Message::Hide => self.state = State::None,
                Message::ShowColors => self.state = State::Colors(Default::default()),
                Message::ShowBeads => {
                    self.state = State::Beads(
                        BeadsWidget {
                            line_ref: self.line_ref.clone(),
                        }
                    );
                }
                Message::GridUpdated(grid) => self.line_ref = grid,
                _ => {}
            }
        }
    }

    pub trait AsBeadsLine :  Debug + AsRef<BeadsLine<Bead<Color>>> {}
    impl <T: AsRef< BeadsLine<Bead<Color>> > + Debug> AsBeadsLine for T {}

    #[derive(Debug)]
    struct BeadsWidget {
        line_ref: Arc<dyn AsBeadsLine>
    }
    const SYMBOLS: [&str;26] = ["A","B","C","D","E","F","G","H","I","J","K","L","M",
                                "N","O","P","Q","R","S","T","U","V","W","X","Y","Z"];

    impl AppWidget for BeadsWidget {
        type Message = Message;

        fn view(&mut self) -> Element<'_, Self::Message> {
            let line = self.line_ref.as_ref().as_ref();
            let summary = line.map(|x|x.color.clone()).summary();
            let mut sorted_summary: Vec<_> = summary.iter().collect();
            let undefined = "?";
            sorted_summary.sort_unstable_by_key(|(&color, _)| { color.to_string() });
            let mut range = SYMBOLS.iter();
            let symbols: HashMap<_,_> = sorted_summary.iter().map(|(obj, _)|{
                (obj.clone(), *range.next().unwrap_or(&undefined))
            }).collect();
            let summary = Column::with_children(sorted_summary.iter().map(|(&color, &count)| {
                Row::new().spacing(5)
                    .push(Text::new(symbols.get(&color).unwrap_or(&undefined).to_string()).width(Length::Units(15)))
                    .push(ColorBox::new(color))
                    .push(Text::new(count.to_string()))
                    .into()
            }).collect()).into();

            let schema = line.line().iter()
                .enumerate()
                .map(|(i, (Bead {color, filled}, count))|{
                    Row::new().spacing(5).align_items(Align::Center)
                        .push(Checkbox::new(
                            *filled,
                            symbols.get(color).unwrap_or(&undefined).to_string(),
                            move |_| Message::ToggleCheckbox(i)
                        ).spacing(1).width(Length::Units(35)))
                        .push(ColorBox::new(color.clone()))
                        .push(Text::new(count.to_string()))
                        .into()
                })
                .collect();

            let schema = Column::with_children(schema).spacing(1).into();
            Column::with_children(vec![
                Text::new(format!("Width: {}", line.width)).into(),
                Text::new("Summary").into(),
                summary,
                Text::new("Schema").into(),
                schema
            ]).into()
        }

        fn update(&mut self, msg: Self::Message) {
            match msg {
                Message::GridUpdated(model) => self.line_ref = model,
                _ => {},
            }
        }
    }

    #[derive(Debug)]
    struct ColorMenu {
        btn_add: button::State,
        btn_remove: button::State,
        hsl: colors::Hsl,
        sliders: (slider::State, slider::State, slider::State),
    }

    impl Default for ColorMenu {
        fn default() -> Self {
            Self {
                btn_add: Default::default(),
                btn_remove: Default::default(),
                hsl: colors::Hsl::default(),
                sliders: Default::default(),
            }
        }
    }

    fn hsl_2_color(hsl: colors::Hsl) -> iced::Color {
        let (r,g,b) = colors::Srgb::from(hsl).into_components();
        iced::Color::from_rgb(r,g,b)
    }
    impl AppWidget for ColorMenu {
        type Message = Message;

        fn view(&mut self) -> Element<'_, Self::Message> {
            let (h_state,s_state,l_state) = &mut self.sliders;
            let (hue, sat, light) = self.hsl.clone().into_components();
            let hue = hue.to_positive_degrees();
            let column = Column::new().width(Length::Units(300))
                .push(Button::new(&mut self.btn_remove, icon::REMOVE.svg())
                    .width(Length::Units(30))
                    .on_press(Message::RemoveColor)
                )
                .push(Element::new(Gradient::Hue))
                .push(slider::Slider::new(
                    h_state,
                    0.0..=3600.0,
                    hue*10.0,
                    |degrees|Message::ConfigColor(ColorPart::Hue(degrees/10.0))
                ))
                .push(Space::new(Length::Fill, Length::Units(5)))

                .push(Element::new(Gradient::Saturation(hue)))
                .push(slider::Slider::new(
                    s_state,
                    0.0..=1.0,
                    sat,
                    |sat|Message::ConfigColor(ColorPart::Saturation(sat))
                ))
                .push(Space::new(Length::Fill, Length::Units(5)))

                .push(Element::new(Gradient::Light {hue, sat}))
                .push(slider::Slider::new(
                    l_state,
                    0.0..=1.0,
                    light,
                    |light|Message::ConfigColor(ColorPart::Lightness(light))
                ));
            let color = hsl_2_color(self.hsl);
            let submit = Container::new(
                Button::new(&mut self.btn_add, icon::ADD.svg()).width(Length::Units(30))
                    .on_press(Message::AddColor(color.into()))
            ).width(Length::Units(100))
                .style(Colored(color));

            Element::new(column.push(submit)).explain(iced::Color::BLACK)

        }

        fn update(&mut self, msg: Self::Message) {
            match msg {
                Message::ConfigColor(part) => {
                    match part {
                        ColorPart::Hue(hue) => self.hsl.hue = colors::RgbHue::from_degrees(hue),
                        ColorPart::Saturation(sat) => self.hsl.saturation = sat,
                        ColorPart::Lightness(lightness) => self.hsl.lightness = lightness,
                    }
                },
                _ => {}
            }
        }
    }
}