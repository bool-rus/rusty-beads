extern crate iced;

mod reimport;
mod field;
mod lib;
mod ui;
mod wrapper;
mod style;
mod pallette;
mod menu;
mod io;

use reimport::*;
use field::Grid;
use ui::AsElement;
use lib::Color;
use lib::Message;
use menu::TopMenu;


#[derive(Default)]
struct Counter {
    // The counter value
    top_menu: TopMenu,
    grid: Grid<Color>,
    active_color: Color,
}



impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Default::default()
    }
    fn title(&self) -> String {
        "test-title".into()
    }
    fn update(&mut self, message: Message) {
        match message {
            Message::PlateClicked(row, col) => {
                self.grid.set(row,col,self.active_color).unwrap_or_else(|e|{
                    println!("Error: {}", e);
                    Default::default()
                });
            }
            Message::SetColor(color) => { self.active_color = color }
            Message::ExportPressed => {
                crate::io::write("grid.csv",&self.grid).unwrap();
            }
            Message::OpenPressed => {
                let grid = crate::io::read("grid.csv").unwrap();
                self.grid = grid;
            }
            Message::GrowPressed => {self.grid.grow(Default::default())}
            Message::ShrinkPressed => {self.grid.shrink().unwrap_or_else(|e| {
                println!("Error: {}", e);
            });}
        }
    }
    fn view(&mut self) -> Element<'_, Message> {
        Column::new()
            .height(Length::Fill)
            .width(Length::Fill)
            .push(self.top_menu.as_element())
            .push(Space::new(Length::Fill, Length::Units(10)))
            .push(Row::new().push(self.grid.as_element()))
            .into()
    }
}

fn main() {
    Counter::run(Settings {
        window: iced::window::Settings {
            size: (500, 500),
            resizable: true,
            decorations: true,
        },
        flags: (),
        default_font: None,
        antialiasing: true,
    });
}
