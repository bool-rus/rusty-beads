
pub mod right {
    use crate::reimport::*;
    use std::rc::Rc;
    use crate::beads::{Beads, create_beads};
    use crate::entities::Color;
    use crate::ui::AppWidget;
    use crate::grid::Grid;
    use crate::ui::widget::ColorBox;
    use std::cell::{RefCell, Cell};

    #[derive(Debug, Copy, Clone)]
    pub enum Message {
        Beads
    }
    #[derive(Debug, Copy, Clone)]
    pub enum State {
        None,
        Beads,
    }

    pub struct RightPanel {
        beads: BeadsWidget,
        scroll: scrollable::State,
        state: Rc<Cell<State>>,
    }

    impl RightPanel {
        pub fn new(grid: Rc<RefCell<Grid<Color>>>, state: Rc<Cell<State>>) -> Self {
            Self {
                beads: BeadsWidget {grid},
                scroll: Default::default(),
                state,
            }
        }
    }

    impl AppWidget for RightPanel {
        type Message = Message;
        type UpdateData = ();

        fn view(&mut self) -> Element<'_, Self::Message> {
            Scrollable::new(&mut self.scroll).push(
                match self.state.get() {
                    State::None => {Space::new(Length::Units(0), Length::Units(0)).into()},
                    State::Beads => {self.beads.view()},
                })
                .into()
        }

        fn update(&mut self, msg: Self::Message) {
            match self.state.get() {
                State::None => {},
                State::Beads => {self.beads.update(msg)},
            }
        }
    }

    struct BeadsWidget {
        pub grid: Rc<RefCell<Grid<Color>>>,
    }

    impl AppWidget for BeadsWidget {
        type Message = Message;
        type UpdateData = ();

        fn view(&mut self) -> Element<'_, Self::Message> {
            let beads = create_beads(self.grid.borrow().as_data());
            Column::with_children(
                beads.iter().map(|(color, count)| {
                    Row::new().spacing(5).align_items(Align::Center)
                        .push(ColorBox::new(color.clone()))
                        .push(Text::new(count.to_string()))
                        .into()
                }).collect()
            ).spacing(1).into()
        }
    }
}