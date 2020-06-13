
pub mod right {
    use crate::reimport::*;
    use std::rc::Rc;
    use crate::beads::{Beads, create_beads};
    use crate::entities::Color;
    use crate::ui::AppWidget;
    use crate::grid::Grid;
    use crate::ui::widget::ColorBox;
    use std::cell::RefCell;

    #[derive(Debug, Copy, Clone)]
    pub enum Message {
        Beads
    }

    pub struct RightPanel {
        grid: Rc<RefCell<Grid<Color>>>,
        beads_scroll: scrollable::State,
    }

    impl RightPanel {
        pub fn new(grid: Rc<RefCell<Grid<Color>>>) -> Self {
            Self {grid, beads_scroll: Default::default()}
        }
    }

    impl AppWidget for RightPanel {
        type Message = Message;
        type UpdateData = ();

        fn view(&mut self) -> Element<'_, Self::Message> {
            let beads = create_beads(self.grid.borrow().as_data());
            let col = Column::with_children(
                beads.iter().map(|(color, count)| {
                    Row::new().spacing(5).align_items(Align::Center)
                        .push(ColorBox::new(color.clone()))
                        .push(Text::new(count.to_string()))
                        .into()
                }).collect()
            ).spacing(1);
            Scrollable::new(&mut self.beads_scroll)
                .push(col).into()
        }
    }
}