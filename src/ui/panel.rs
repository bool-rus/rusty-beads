use super::menu::right::Message as RightMenuMessage;
use super::menu::left::Message as LeftMenuMessage;
use super::files;

pub mod left {
    use crate::reimport::*;
    use crate::ui::AppWidget;
    use std::num::ParseIntError;
    use super::LeftMenuMessage as MenuMsg;
    use super::files::Message as FilesMessage;
    use super::files::FSMenu;
    use std::path::PathBuf;
    use crate::io::default_dir;

    #[derive(Debug, Copy, Clone)]
    pub enum Message {
        Ignore,
        ShowResize,
        ShowOpen,
        ShowSave,
        Hide,
        Resize(usize, usize),
        InputWidth(usize),
        InputHeight(usize),
        WrongValue,
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
        FS(FSMenu),
    }

    pub struct Panel {
        state: State,
    }

    impl Panel {
        pub fn selected_path(&self) -> Option<PathBuf> {
            match &self.state {
                State::FS(widget) => Some(widget.selected()),
                _ => None
            }
        }
    }

    impl Default for Panel {
        fn default() -> Self {
            Self { state: State::Empty }
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
                ShowResize => { self.state = State::Resize(Default::default())},
                ShowOpen => { self.state = State::FS(FSMenu::open(default_dir()))},
                ShowSave => { self.state = State::FS(FSMenu::save(default_dir()))},
                msg => {
                    match self.state {
                        State::Empty => {},
                        State::Resize(ref mut widget) => {widget.update(msg)},
                        State::FS(ref mut widget) => {
                            match msg {
                                Message::FS(msg) => {widget.update(msg)},
                                _ => {}
                            }
                        }
                    }
                    match msg {
                        Message::Resize(_,_) => self.state = State::Empty,
                        _ => {}
                    }
                }
            }
        }
    }
    #[derive(Default)]
    pub struct ResizeWidget {
        input_width: text_input::State,
        input_height: text_input::State,
        width: String,
        height: String,
        btn_resize: button::State,
    }


    fn resize_message(width: &str, height: &str) -> Result<Message, ParseIntError> {
        Ok(Message::Resize(width.parse()?, height.parse()?))
    }

    impl AppWidget for ResizeWidget {
        type Message = Message;
        fn view(&mut self) -> Element<'_, Self::Message> {
            let width_field = TextInput::new(
                &mut self.input_width,
                &"10",
                &self.width,
                |s| { s.parse().map_or(Message::WrongValue, |n| Message::InputWidth(n)) },
            );
            let height_field = TextInput::new(
                &mut self.input_height,
                &"10",
                &self.height,
                |s| { s.parse().map_or(Message::WrongValue, |n| Message::InputHeight(n)) },
            );

            Row::new()
                .push(Column::new()
                    .push(Text::new("Width: "))
                    .push(Text::new("Height: "))
                ).push(Column::new().width(Length::Units(50))
                .push(width_field)
                .push(height_field)
                .push(Button::new(&mut self.btn_resize, Text::new("OK"))
                    .on_press(resize_message(&self.width, &self.height).unwrap_or(Message::WrongValue))
                )
            ).into()
        }

        fn update(&mut self, msg: Self::Message) {
            match msg {
                Message::Resize(_, _) => {/* top level process */},
                Message::InputWidth(s) => { self.width = s.to_string(); },
                Message::InputHeight(s) => { self.height = s.to_string(); },
                Message::WrongValue => {},
                _ => {}
            }
        }
    }
}

pub mod right {
    use crate::reimport::*;
    use std::rc::Rc;
    use crate::beads::{BeadsLine, BeadsLineBuilder};
    use crate::entities::{Color, Schema};
    use crate::ui::AppWidget;
    use crate::grid::Grid;
    use crate::ui::widget::ColorBox;
    use std::cell::{RefCell, Cell};
    use super::RightMenuMessage as MenuMsg;
    use std::hash::Hash;
    use std::collections::HashMap;

    #[derive(Debug, Copy, Clone)]
    pub enum Message {
        Ignore,
        ShowBeads,
        Hide,
        GridChanged,
        ToggleCheckbox(usize),
    }

    #[derive(Debug)]
    pub enum State {
        None,
        Beads(BeadsWidget),
    }

    pub struct RightPanel {
        grid: Rc<RefCell<Grid<Color>>>,
        scroll: scrollable::State,
        state: State,
        schema: Rc<Cell<Schema>>,
    }

    impl RightPanel {
        pub fn new(grid: Rc<RefCell<Grid<Color>>>, schema: Rc<Cell<Schema>>) -> Self {
            Self {
                grid,
                scroll: Default::default(),
                state: State::None,
                schema,
            }
        }
        pub fn refresh(&mut self) {
            match self.state {
                State::None => {}
                State::Beads(_) => {
                    let line =  match self.schema.get() {
                        Schema::FirstOffset => BeadsLineBuilder::RLOffset(true),
                        Schema::SecondOffset => BeadsLineBuilder::RLOffset(false),
                        Schema::Straight => BeadsLineBuilder::RLSquare,
                    }.build(self.grid.borrow().as_table());
                    self.state = State::Beads(BeadsWidget::new(self.grid.borrow().width(), line))
                }
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
                })
                .into()
        }

        fn update(&mut self, msg: Self::Message) {
            match (&mut self.state, msg) {
                (_, Message::Hide) => { self.state = State::None }
                (_, Message::ShowBeads) => {
                    self.state = State::Beads(BeadsWidget::empty());
                    self.refresh();
                }
                (_, Message::GridChanged) => { self.refresh() }
                (State::Beads(ref mut widget), msg) => { widget.update(msg) }
                (State::None, _) => {}
                (_, Message::ToggleCheckbox(_)) => {}
            }
        }
    }

    #[derive(Debug)]
    struct BeadsWidget {
        line_width: usize,
        line: BeadsLine<Color>,
        checkboxes: Vec<bool>,
    }

    impl BeadsWidget {
        fn new(line_width: usize, line: BeadsLine<Color>) -> Self {
            Self { line_width, checkboxes: vec![false; line.line().len()], line }
        }
        fn empty() -> Self {
            Self { line_width: 0, line: BeadsLineBuilder::RLOffset(true).build(Vec::new()), checkboxes: Vec::new() }
        }
    }

    const SYMBOLS: [&str;26] = ["A","B","C","D","E","F","G","H","I","J","K","L","M",
                                "N","O","P","Q","R","S","T","U","V","W","X","Y","Z"];

    impl AppWidget for BeadsWidget {
        type Message = Message;

        fn view(&mut self) -> Element<'_, Self::Message> {
            let mut sorted_summary: Vec<_> = self.line.summary().iter().collect();
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
            let line = Column::with_children(
                self.line.line().iter()
                    .zip(self.checkboxes.iter().enumerate())
                    .map(|((color, count), (i, checked))| {
                        Row::new().spacing(5).align_items(Align::Center)
                            .push(Checkbox::new(
                                *checked,
                                symbols.get(&color).unwrap_or(&undefined).to_string(),
                                move |_x| Message::ToggleCheckbox(i)
                            ).spacing(1).width(Length::Units(35)))
                            .push(ColorBox::new(color.clone()))
                            .push(Text::new(count.to_string()))
                            .into()
                    }).collect()
            ).spacing(1).into();
            Column::with_children(vec![
                Text::new(format!("Width: {}", self.line_width)).into(),
                Text::new("Summary").into(),
                summary,
                Text::new("Scheme").into(),
                line
            ]).into()
        }

        fn update(&mut self, msg: Self::Message) {
            match msg {
                Message::GridChanged => {}
                Message::ToggleCheckbox(i) => {
                    //TODO: обработать none
                    let checked = self.checkboxes.get_mut(i).unwrap();
                    *checked = !*checked;
                }
                _ => {}
            }
        }
    }
}