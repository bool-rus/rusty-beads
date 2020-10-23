mod reimport;
mod grid;
mod entities;
mod ui;
mod wrapper;
mod io;
mod beads;
mod message;
mod service;
mod model;

use reimport::*;
use message::Message;
use ui::*;
use std::cell::{Cell};
use crate::service::AppService;
use std::rc::Rc;
use crate::model::Model;
use std::sync::Arc;


struct App {
    service: AppService,
    top_menu: TopMenu,
    grid_plate: GridPlate,
    right_panel: RightPanel,
    right_menu: RightMenu,
    left_menu: LeftMenu,
    left_panel: LeftPanel,
    mouse_hold: Rc<Cell<bool>>,
}

impl Default for App {
    fn default() -> Self {
        let mouse_hold = Rc::new(Cell::new(false));
        let model = Arc::new(Model::default());
        Self {
            service: Default::default(),
            top_menu: Default::default(),
            grid_plate: GridPlate::new(mouse_hold.clone()),
            right_panel: RightPanel::new(model.clone()),
            right_menu: RightMenu::default(),
            left_menu: LeftMenu::default(),
            mouse_hold,
            left_panel: Default::default(),
        }
    }
}

impl App {
    fn update_children(&mut self, message: Message) {
        self.top_menu.update(message.clone().into());
        self.right_menu.update(message.clone().into());
        self.left_menu.update(message.clone().into());
        self.grid_plate.update(message.clone().into());
        self.left_panel.update(message.clone().into());
        self.right_panel.update(message.clone().into());
    }
}

impl Sandbox for App {
    type Message = Message;

    fn new() -> Self {
        Default::default()
    }
    fn title(&self) -> String {
        "Beads and threads by Bool".into()
    }
    fn update(&mut self, message: Message) {
        if let Some(service_msg) = self.service.process(message.clone().into()) {
            self.update_children(service_msg);
        }
        self.update_children(message.clone());
    }

    fn view(&mut self) -> Element<'_, Message> {
        let active_color = self.top_menu.palette().active_color();
        let top = Container::new(self.top_menu.view().map(From::from))
            .height(Length::Units(30));
        let bottom = Container::new(Text::new(""));
        let left = Container::new(self.left_menu.view().map(From::from))
            .width(Length::Units(30));
        let right = Container::new(self.right_menu.view().map(From::from))
            .width(Length::Units(25));
        let content = Container::new(self.grid_plate.view().map(move |msg| {
            match msg { //TODO: как-то неочевидно, надо переделать
                GridMessage::GridClicked(coord) => Message::Grid(GridMessage::SetColor(coord, active_color)),
                msg => Message::Grid(msg)
            }
        }));
        let row = Row::new().spacing(5)
            .push(Element::new(ui::MouseListener::new(self.mouse_hold.clone())))
            .width(Length::Fill)
            .height(Length::Fill)
            .push(left)
            .push(self.left_panel.view().map(From::from))
            .push(content.height(Length::Fill).width(Length::Fill))
            .push(self.right_panel.view().map(From::from))
            .push(right);
        Column::new().height(Length::Fill).spacing(5)
            .push(top)
            .push(row)
            .push(bottom).into()

    }
}

fn main() {
    App::run(Settings {
        window: iced::window::Settings {
            size: (550, 480),
            resizable: true,
            decorations: true,
        },
        flags: (),
        default_font: None,
        antialiasing: false,
    });
}
