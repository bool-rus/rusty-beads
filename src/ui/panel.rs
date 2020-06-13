
pub mod right {
    use crate::reimport::*;
    use std::rc::Rc;
    use crate::beads::Beads;
    use crate::entities::Color;
    use crate::ui::AppWidget;
    use crate::grid::Grid;
    use crate::ui::widget::ColorBox;

    #[derive(Debug, Copy, Clone)]
    pub enum Message {
        Beads
    }

    #[derive(Default)]
    pub struct RightPanel {
        beads: Option<Beads<Color>>,
        beads_scroll: scrollable::State,
    }

    impl AppWidget for RightPanel {
        type Message = Message;
        type UpdateData = Grid<Color>;

        fn view(&mut self) -> Element<'_, Self::Message> {
            if let Some(ref mut beads) = self.beads {
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
            } else {unimplemented!()}
        }

        fn update_data(&mut self, data: &Self::UpdateData) {
            self.beads = Some(data.into())
        }
    }
}