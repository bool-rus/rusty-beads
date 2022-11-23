#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use egui::{Color32, RichText, Ui, Stroke, Rounding, vec2, Vec2, Sense, Slider, ScrollArea};
use model::{Model, Bead, BeadsLine, Coord, beads::BeadsRow};

mod wrapper;
mod model;
mod palette;

fn main() {

    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Rusty Beads",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

#[derive(Default)]
struct MyApp {
    bead_line: BeadsLine<Bead<Color32>>,
    rotation: isize,
    draw_options: DrawOptions,
    palette: palette::Palette,
}


struct DrawOptions {
    size: Vec2,
    stroke: Stroke,
    rounding: Rounding,
}

impl Default for DrawOptions {
    fn default() -> Self {
        Self { 
            size: vec2(10.0, 10.0), 
            stroke: Stroke::new(0.4, Color32::WHITE), 
            rounding: Default::default(), 
        }
    }
}

struct Colors(Vec<Color32>);

impl Default for Colors {
    fn default() -> Self {
        use Color32 as C;
        Self (vec![C::default(), C::WHITE, C::BLACK, C::RED, C::BLUE, C::GREEN])
    }
}

struct ColorBox<'a> {
    options: &'a DrawOptions,
    bead: &'a Bead<Color32>,
    drawing_color: &'a Option<Color32>,
}

impl <'a> egui::Widget for ColorBox<'a> {
    fn ui(self, ui: &mut Ui) -> egui::Response {
        let (rect, mut response) = ui.allocate_at_least(self.options.size, Sense::hover());
        let mut color = self.bead.color;
        if let Some(drawing_color) = self.drawing_color {
            if response.hovered() && drawing_color != &self.bead.color {
                response.mark_changed();
                color = *drawing_color;
            }
        }
        ui.painter().rect(
            rect,
            self.options.rounding,
            color,
            self.options.stroke,
        );
        response
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top").show(ctx, |ui|{ 
            ui.horizontal_centered(|ui| {
                self.palette.show(ui);
            })
        });
        egui::TopBottomPanel::bottom("bottom").show(ctx, |ui| {
            let delta = ui.input().scroll_delta;
            self.rotation += (delta.x/3.0) as isize;
            let w = self.bead_line.width() as isize;
            if self.rotation.abs() > w {
                self.rotation = self.rotation % w;
            }
            ui.style_mut().spacing.slider_width = ui.available_width();
            ui.add(Slider::new(&mut self.rotation, -w..=w).show_value(false));
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            let drawing = ctx.input().pointer.any_down();
            let drawing_color = if drawing {
                Some(self.palette.active_color())
            } else {
                None
            };
            ui.spacing_mut().icon_spacing = 0.0;
            ui.spacing_mut().item_spacing = vec2(0.0, 0.0);
            let height = self.bead_line.height;
            ScrollArea::vertical().enable_scrolling(!drawing)
                .show_rows(ui, self.draw_options.size.y, height, |ui, range|{
                    ui.horizontal_wrapped(|ui|{
                        ui.spacing_mut().item_spacing = vec2(0.0, 0.0);
                        ui.set_row_height(self.draw_options.size.y);
                        let box_width = self.draw_options.size.x;
                        let offset_tail = box_width / self.bead_line.schema.base() as f32;
                        let max_width = ui.available_width() - ui.spacing().scroll_bar_width - offset_tail;
                        let coord = self.bead_line.table(self.rotation)
                            .skip(range.start)
                            .take(range.end - range.start)
                            .fold(None, |mut coord, row| {
                            let BeadsRow { row, offset, iter } = row;
                            let mut usage = offset_tail * offset as f32;
                            ui.add_space(usage);
                            for (ncol, bead) in iter {
                                usage += box_width;
                                if usage > max_width {
                                    break;
                                }
                                if ui.add(ColorBox{options: &self.draw_options, bead, drawing_color: &drawing_color}).changed() {
                                    coord = Some(Coord{ x: ncol, y: row });
                                }
                            }
                            ui.add_space(offset_tail);
                            ui.end_row();
                            coord
                        }); 
                        if let (Some(coord), Some(ref color)) = (coord, drawing_color){
                            self.bead_line.set_value(color.into(), coord);
                        }
                    });
                });
        });
    }
}

/*
#![windows_subsystem = "windows"]
mod reimport;
mod model;
mod ui;
mod wrapper;
mod io;
mod message;
mod service;

use reimport::*;
use message::Message;
use ui::*;
use service::AppService;
use model::{Model, Color};
use std::sync::Arc;

fn default_colors() -> Vec<Color> {
    vec![
        Color { r: 0x61 ,   g: 0x00,    b: 0x00 },
        Color { r: 0xff,    g: 0x00,    b: 0x88 },
        Color { r: 0x98,    g: 0x02,    b: 0x2f },
        Color { r: 0xcf,    g: 0x32,    b: 0x00 },
        Color { r: 0xff,    g: 0x32,    b: 0x32 },
        Color { r: 0xfd,    g: 0x8c,    b: 0x0e },
        Color { r: 0xff,    g: 0xe6,    b: 0x11 },
        Color { r: 0xff,    g: 0xfc,    b: 0x72 },
        Color { r: 0x88,    g: 0x0c,    b: 0x14 },
        Color { r: 0xb0,    g: 0x5e,    b: 0x07 },
        Color { r: 0x29,    g: 0x13,    b: 0x9c },
        Color { r: 0x3f,    g: 0x9b,    b: 0xe3 },
        Color { r: 0x69,    g: 0xc7,    b: 0xac },
        Color { r: 0x9a,    g: 0xcc,    b: 0xb0 },
        Color { r: 0x49,    g: 0x8c,    b: 0x55 },
        Color { r: 0x00,    g: 0xb1,    b: 0x5a },
        Color { r: 0x3e,    g: 0xe0,    b: 0x19 },
        Color { r: 0x8d,    g: 0xe4,    b: 0x6f },
        Color { r: 0x8c,    g: 0x62,    b: 0xd3 },
        Color { r: 0xc8,    g: 0xb5,    b: 0xff },
        Color { r: 0xdd,    g: 0xdd,    b: 0xdd },
        Color { r: 0x90,    g: 0x93,    b: 0x9e },
        Color { r: 0x00,    g: 0x00,    b: 0x00 },
    ]
}

struct App {
    service: AppService,
    top_menu: TopMenu,
    grid_plate: GridPlate<Model<Color>>,
    right_panel: RightPanel,
    right_menu: RightMenu,
    left_menu: LeftMenu,
    left_panel: LeftPanel,
}

impl Default for App {
    fn default() -> Self {
        let model = default_colors().into_iter().fold(Model::default(), |mut model, color| {
            model.add_color(color);
            model
        });
        let service = AppService::new(model.clone());
        let model = Arc::new(model);
        Self {
            service,
            top_menu: TopMenu::new(model.clone()),
            grid_plate: GridPlate::new(model.clone()),
            right_panel: RightPanel::new(model.clone()),
            right_menu: RightMenu::default(),
            left_menu: LeftMenu::default(),
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
        let top = Container::new(self.top_menu.view().map(From::from))
            .height(Length::Units(30));
        let bottom = Container::new(Text::new(""));
        let left = Container::new(self.left_menu.view().map(From::from))
            .width(Length::Units(30));
        let right = Container::new(self.right_menu.view().map(From::from))
            .width(Length::Units(25));
        let content = Container::new(self.grid_plate.view().map(From::from));
        let row = Row::new().spacing(5)
            .push(Element::new(ui::MouseListener(Message::MouseRelease)))
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

*/