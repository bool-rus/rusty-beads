use crate::reimport::*;
use button::State;
use crate::pallette::Pallette;
use crate::ui::AsElement;
use crate::lib::Message;

#[derive(Default)]
pub struct TopMenu {
    palette: Pallette,
    grow: State,
    shrink: State,
    export: State,
    load: State,
}

impl AsElement for TopMenu {
    fn as_element(&mut self) -> Element<'_, Message> {
        Row::new()
            .push(Button::new(&mut self.load, Text::new("Load")).on_press(Message::OpenPressed))
            .push(Button::new(&mut self.export, Text::new("Export")).on_press(Message::ExportPressed))
            .push(Button::new(&mut self.grow, Text::new("+")).on_press(Message::GrowPressed))
            .push(Button::new(&mut self.shrink, Text::new("-")).on_press(Message::ShrinkPressed))
            .push(self.palette.as_element())
            .spacing(5)
            .into()
    }
}