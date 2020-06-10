use crate::reimport::*;
use button::State;
use crate::pallette::Pallette;
use crate::ui::AsContainer;
use crate::lib::Message;

#[derive(Default)]
pub struct TopMenu {
    palette: Pallette,
    grow: State,
    shrink: State,
    export: State,
    load: State,
}

impl AsContainer for TopMenu {
    fn as_container(&mut self) -> Container<'_, Message> {
        Container::new(Row::new()
            .push(Button::new(&mut self.load, Text::new("Load")).on_press(Message::OpenPressed))
            .push(Button::new(&mut self.export, Text::new("Export")).on_press(Message::ExportPressed))
            .push(Button::new(&mut self.grow, Text::new("+")).on_press(Message::GrowPressed))
            .push(Button::new(&mut self.shrink, Text::new("-")).on_press(Message::ShrinkPressed))
            .push(self.palette.as_container())
            .spacing(5))
    }
}