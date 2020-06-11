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
mod beads;
mod icon;

use reimport::*;
use field::Grid;
use ui::AsContainer;
use lib::Color;
use lib::Message;
use menu::*;
use beads::Beads;


#[derive(Default)]
struct Counter {
    top_menu: TopMenu,
    right_menu: RightMenu,
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
            Message::BeadsPressed => {
                self.right_menu.beads_pressed()
            }
        }
        self.right_menu.update(&self.grid);
    }
    fn view(&mut self) -> Element<'_, Message> {
        let top = self.top_menu.as_container();
        let bottom = Container::new(Text::new("footer"));
        let left = Container::new(Column::new()
            .push(Text::new("L"))
            .push(Text::new("E"))
            .push(Text::new("F"))
            .push(Text::new("T"))
        );
        let content = self.grid.as_container();
        Column::new().height(Length::Fill).spacing(10)
            .push(top)
            .push(Row::new()
                .width(Length::Fill)
                .height(Length::Fill)
                .push(left)
                .push(content.height(Length::Fill).width(Length::Fill))
                .push(self.right_menu.as_container())
            ).push(bottom).into()

    }
}

fn main() {
    Counter::run(Settings {
        window: iced::window::Settings {
            size: (480, 480),
            resizable: true,
            decorations: true,
        },
        flags: (),
        default_font: None,
        antialiasing: true,
    });
}
