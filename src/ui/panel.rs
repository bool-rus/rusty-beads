use super::menu::right::Message as RightMenuMessage;


pub mod right {
    use crate::reimport::*;
    use std::rc::Rc;
    use crate::beads::{BeadsLine, BeadsLineBuilder};
    use crate::entities::Color;
    use crate::ui::AppWidget;
    use crate::grid::Grid;
    use crate::ui::widget::ColorBox;
    use std::cell::{RefCell, Cell};
    use super::RightMenuMessage as MenuMsg;
    use std::hash::Hash;

    #[derive(Debug, Copy, Clone)]
    pub enum Message {
        Menu(MenuMsg),
        GridChanged,
        Toggle(usize),
    }

    impl From<MenuMsg> for Message {
        fn from(msg: MenuMsg) -> Self {
            Message::Menu(msg)
        }
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
        first_offset: Rc<Cell<bool>>,
    }

    impl RightPanel {
        pub fn new(grid: Rc<RefCell<Grid<Color>>>, first_offset: Rc<Cell<bool>>) -> Self {
            Self {
                grid,
                scroll: Default::default(),
                state: State::None,
                first_offset,
            }
        }
        pub fn refresh(&mut self) {
            match self.state {
                State::None => {},
                State::Beads(_) => {
                    let line = BeadsLineBuilder::RLOffset(self.first_offset.get()).build(self.grid.borrow().as_table());
                    self.state = State::Beads(BeadsWidget::new(line))
                },
            }
        }
    }

    impl AppWidget for RightPanel {
        type Message = Message;

        fn view(&mut self) -> Element<'_, Self::Message> {
            Scrollable::new(&mut self.scroll).push(
                match self.state {
                    State::None => {Space::new(Length::Units(0), Length::Units(0)).into()},
                    State::Beads(ref mut widget) => {widget.view()},
                })
                .into()
        }

        fn update(&mut self, msg: Self::Message) {
            match (&mut self.state, msg) {
                (_, Message::Menu(MenuMsg::Hide)) => {self.state = State::None},
                (_, Message::Menu(MenuMsg::ShowBeads)) => {
                    self.state = State::Beads(BeadsWidget::empty());
                    self.refresh();
                },
                (_, Message::GridChanged) => { self.refresh() },
                (State::Beads(ref mut widget), msg) => {widget.update(msg)},
                (State::None, _) => {},
                (_, Message::Toggle(_)) => {}
            }
        }
    }
    #[derive(Debug)]
    struct BeadsWidget {
        line: BeadsLine<Color>,
        checkboxes: Vec<bool>,
    }

    impl BeadsWidget {
        fn new(line: BeadsLine<Color>) -> Self {
            Self {checkboxes: vec![false; line.line().len()], line}
        }
        fn empty() -> Self {
            Self {line: BeadsLineBuilder::Empty.build(Vec::new()), checkboxes: Vec::new()}
        }
    }

    impl AppWidget for BeadsWidget {
        type Message = Message;

        fn view(&mut self) -> Element<'_, Self::Message> {
            let mut sorted_summary: Vec<_> = self.line.summary().iter().collect();
            sorted_summary.sort_unstable_by_key(|(&color, _)|{color.to_string()});
            let summary = Column::with_children(sorted_summary.iter().map(|(&color, &count)|{
                Row::new().spacing(5)
                    .push(ColorBox::new(color))
                    .push(Text::new(count.to_string()))
                    .into()
            }).collect()).into();
            let line = Column::with_children(
                self.line.line().iter()
                    .zip(self.checkboxes.iter().enumerate())
                    .map(|((color, count), (i,checked))| {
                    Row::new().spacing(5).align_items(Align::Center)
                        .push(Checkbox::new(*checked, String::new(), move |_x|Message::Toggle(i)))
                        .push(ColorBox::new(color.clone()))
                        .push(Text::new(count.to_string()))
                        .into()
                }).collect()
            ).spacing(1).into();
            Column::with_children(vec![
                Text::new("Summary").into(),
                summary,
                Text::new("Scheme").into(),
                line
            ]).into()
        }

        fn update(&mut self, msg: Self::Message) {
            match msg {
                Message::Menu(_) => {},
                Message::GridChanged => {},
                Message::Toggle(i) => {
                    //TODO: обработать none
                    let checked = self.checkboxes.get_mut(i).unwrap();
                    *checked = !*checked;
                },
            }
        }
    }
}