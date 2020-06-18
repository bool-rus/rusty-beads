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
use std::rc::Rc;
use std::cell::{RefCell, Cell};


struct Counter {
    grid: Rc<RefCell<Grid<Color>>>,
    top_menu: TopMenu,
    grid_plate: GridPlate,
    right_panel: RightPanel,
    right_menu: RightMenu,
    active_color: Color,
}

impl Default for Counter {
    fn default() -> Self {
        let grid = Rc::new(RefCell::new(Default::default()));
        let right_panel_state = Rc::new(Cell::new(RightPanelState::None));
        Self {
            grid: grid.clone(),
            top_menu: Default::default(),
            grid_plate: GridPlate::new(grid.clone()),
            right_panel: RightPanel::new(grid.clone(), right_panel_state.clone()),
            right_menu: RightMenu::new(right_panel_state.clone()),
            active_color: Default::default(),
        }
    }
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
                        crate::io::write("grid.csv", self.grid.borrow().as_table()).unwrap();
                    }
                    TopMenuMessage::OpenPressed => {
                        let grid = crate::io::read("grid.csv").unwrap();
                        self.grid.borrow_mut().update_from_another(grid);
                        self.right_panel.update(RightPanelMessage::GridChanged);
                    }
                    TopMenuMessage::GrowPressed => {
                        self.grid.borrow_mut().grow(Default::default()) ;
                        self.right_panel.update(RightPanelMessage::GridChanged);
                    }
                    TopMenuMessage::ShrinkPressed => {
                        self.grid.borrow_mut().shrink().unwrap_or_else(|e| {
                            println!("Error: {}", e);
                        });
                        self.right_panel.update(RightPanelMessage::GridChanged);
                    }
                    TopMenuMessage::Palette(msg) => match msg {
                        PaletteMessage::SetColor(color) => { self.active_color = color }
                    }
                }
            }
            Message::Grid(msg) => {
                self.grid_plate.update(msg);
                match msg {
                    GridMessage::GridClicked(row, col) => {
                        self.grid_plate.update(GridMessage::SetColor(row, col,self.active_color))
                    },
                    _ => {}
                }
                self.right_panel.update(RightPanelMessage::GridChanged);
            }
            Message::RightMenu(msg) => {
                self.right_menu.update(msg);
                self.right_panel.update(msg.into());
            }
            Message::RightPanel(msg) => {
                self.right_panel.update(msg);
            }
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
        let content = Container::new(self.grid_plate.view().map(From::from));
        let mut row = Row::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .push(left)
            .push(content.height(Length::Fill).width(Length::Fill))
            .push(self.right_panel.view().map(From::from))
            .push(right);
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
        antialiasing: false,
    });
}
