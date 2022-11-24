#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
use eframe::egui;
use egui::*;
use model::*;
use beads::BeadsRow;
use settings::DrawOptions;

mod wrapper;
mod model;
mod palette;
mod settings;
mod summary;

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

    show_draw_options: bool,
    show_summary: bool,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.bead_line.show_summary(ctx, &mut self.show_summary);
        self.draw_options.show(ctx, &mut self.show_draw_options);
        egui::TopBottomPanel::top("top").show(ctx, |ui|{ 
            ui.horizontal_centered(|ui| {
                ui.toggle_value(&mut self.show_draw_options, "⛭");
                ui.toggle_value(&mut self.show_summary, "");
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
                                let is_seam = ncol == 0;
                                if ui.add(ColorBox{options: &self.draw_options, bead, drawing_color: &drawing_color, is_seam}).changed() {
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


struct ColorBox<'a> {
    options: &'a DrawOptions,
    bead: &'a Bead<Color32>,
    drawing_color: &'a Option<Color32>,
    is_seam: bool,
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
        let stroke = if self.bead.filled {
            self.options.filled_stroke
        } else {
            self.options.stroke
        };
        let rounding = if self.is_seam {
            self.options.seam_rounding
        } else {
            self.options.rounding
        };
        ui.painter().rect(
            rect,
            rounding,
            color,
            stroke,
        );
        response
    }
}
