extern crate iced;

mod reimport;
mod grid;
mod entities;
mod ui;
mod wrapper;
mod io;
mod beads;
mod message;

use reimport::*;
use grid::Grid;
use entities::Color;
use message::Message;
use ui::*;


#[derive(Default)]
struct Counter {
    top_menu: TopMenu,
    right_menu: RightMenu,
    grid: Grid<Color>,
    right_panel: Option<RightPanel>,
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
            Message::TopMenu(msg) => {
                self.top_menu.update(msg);
                match msg {
                    TopMenuMessage::ExportPressed => {
                        crate::io::write("grid.csv", &self.grid).unwrap();
                    }
                    TopMenuMessage::OpenPressed => {
                        let grid = crate::io::read("grid.csv").unwrap();
                        self.grid = grid;
                    }
                    TopMenuMessage::GrowPressed => { self.grid.grow(Default::default()) }
                    TopMenuMessage::ShrinkPressed => {
                        self.grid.shrink().unwrap_or_else(|e| {
                            println!("Error: {}", e);
                        });
                    }
                    TopMenuMessage::Palette(msg) => match msg {
                        PaletteMessage::SetColor(color) => { self.active_color = color }
                    }
                }
            }
            Message::Grid(msg) => {
                self.grid.update(msg);
                match msg {
                    GridMessage::GridClicked(row, col) => {
                        self.grid.set(row,col,self.active_color).unwrap_or_else(|e|{
                            println!("Error: {}", e);
                            Default::default()
                        });
                    }
                }
            }
            Message::RightMenu(msg) => {
                self.right_menu.update(msg);
                if self.right_menu.show_beads() {
                    self.right_panel = Some(Default::default())
                } else {
                    self.right_panel = None
                }
            }
            Message::RightPanel(msg) => if let Some(ref mut panel) = self.right_panel { panel.update(msg) }
        }
        self.top_menu.update_data(&());
        self.grid.update_data(&());
        self.right_menu.update_data(&());
        if let Some(ref mut right_panel) = self.right_panel {
            right_panel.update_data(&self.grid)
        }
    }
    fn view(&mut self) -> Element<'_, Message> {
        let top = self.top_menu.view().map(From::from);
        let bottom = Container::new(Text::new("footer"));
        let left = Container::new(Column::new()
            .push(Text::new("L"))
            .push(Text::new("E"))
            .push(Text::new("F"))
            .push(Text::new("T"))
        );
        let right = Container::new(self.right_menu.view().map(From::from))
            .width(Length::Units(25));
        let content = Container::new(self.grid.view().map(From::from));
        let mut row = Row::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .push(left)
            .push(content.height(Length::Fill).width(Length::Fill));
        if let Some(ref mut panel) = self.right_panel {
            row = row.push(panel.view().map(From::from))
        };
        row = row.push(right);
        Column::new().height(Length::Fill).spacing(10)
            .push(top)
            .push(row)
            .push(bottom).into()

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
