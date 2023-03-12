use super::*;

#[derive(Default)]
pub struct BeadApp {
    beads: Model,
    rotation: isize,
    draw_options: Settings,
    palette: palette::Palette,
    drawing: bool,
    prev_coord: Option<Coord>,
    undo: usize,
    show_draw_options: bool,
    show_summary: bool,
    #[cfg(target_arch="wasm32")]
    waiting_file: bool,
}

impl BeadApp {
    fn update_from_line(&mut self, line: BeadsLine<Color32>) {
        let colors = line.summary().keys().copied().collect();
        self.palette.set_colors(colors);
        self.beads = line.into();
    }
}

#[cfg(not(target_arch="wasm32"))]
impl BeadApp {
    fn open_file(&mut self) {
        match io::open_file() {
            Ok(line) => {
                self.update_from_line(line);
            },
            Err(e) => println!("{e}"),
        }
    }
    fn on_update(&mut self) {

    }
}
#[cfg(target_arch="wasm32")]
impl BeadApp {
    fn open_file(&mut self) {
        self.waiting_file = true;
        io::open_file();
    }
    fn on_update(&mut self) {
        if self.waiting_file {
            if let Some(beads) = io::invoke_beads() {
                self.waiting_file = false;
                if let Ok(line) = beads {
                    self.update_from_line(line);
                }
            } 
        }
    }
}

impl eframe::App for BeadApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.on_update();
        if let Some(scale) = self.draw_options.changed_font_scale() {
            let mut style = ctx.style().as_ref().clone();
            style.text_styles.iter_mut().for_each(|(_, font)|font.size *= scale);
            ctx.set_style(style);
        }
        self.beads.line_mut().show_summary(ctx, &mut self.show_summary);
        self.draw_options.show(ctx, &mut self.show_draw_options, &mut self.beads);
        egui::TopBottomPanel::top("top").show(ctx, |ui|{ 
            ui.horizontal(|ui| {
                if ui.button(text4btn("📂")).clicked() {
                    self.open_file();
                }
                if ui.button(text4btn("💾")).clicked() {
                    if let Some(e) = io::save_file(self.beads.line()).err() {
                        println!("{e}");
                    }
                }
                ui.toggle_value(&mut self.show_draw_options, text4btn("⛭"));
                ui.toggle_value(&mut self.show_summary, text4btn("🍡")); // //🏮 // 
                if ui.button(text4btn("⟲")).clicked() {
                    self.beads.undo();
                }
                if ui.add(Slider::new(&mut self.undo, self.beads.max_undo()..=0).show_value(false).logarithmic(true)).changed() {
                    self.beads.undo_at(self.undo);
                }
                if ui.button(text4btn("⟳")).clicked() {
                    self.beads.redo();
                }
                self.palette.show(ui);
            })
        });
        egui::TopBottomPanel::bottom("bottom").show(ctx, |ui| {
            let delta = ui.input().scroll_delta;
            self.rotation += (delta.x/3.0) as isize;
            let w = self.beads.line().width() as isize;
            if self.rotation.abs() > w {
                self.rotation = self.rotation % w;
            }
            ui.style_mut().spacing.slider_width = ui.available_width();
            ui.add(Slider::new(&mut self.rotation, -w..=w).show_value(false));
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            let pointer = ctx.input().pointer.clone();
            if pointer.any_released() {
                self.drawing = false;
                self.prev_coord = None;
            }
            let drawing_color = if self.drawing {
                Some(self.palette.active_color())
            } else {
                None
            };
            ui.spacing_mut().icon_spacing = 0.0;
            ui.spacing_mut().item_spacing = vec2(0.0, 0.0);
            let height = self.beads.height();
            ScrollArea::vertical().enable_scrolling(!self.drawing)
                .show_rows(ui, self.draw_options.size.y, height, |ui, range|{
                    ui.horizontal_wrapped(|ui|{
                        ui.spacing_mut().item_spacing = vec2(0.0, 0.0);
                        ui.set_row_height(self.draw_options.size.y);
                        let mut drawing = false;
                        let box_width = self.draw_options.size.x;
                        let offset_tail = box_width / self.beads.line().schema.base() as f32;
                        let max_width = ui.available_width() - ui.spacing().scroll_bar_width - offset_tail;
                        let coord = self.beads.line().table(self.rotation, range.start)
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
                                let response = ui.add(ColorBox{options: &self.draw_options, bead, drawing_color: &drawing_color, is_seam});
                                if response.changed() {
                                    coord = Some(Coord{ x: ncol, y: row });
                                }
                                if response.hovered() && pointer.any_pressed() {
                                    drawing = true;
                                }
                            }
                            ui.add_space(offset_tail);
                            ui.end_row();
                            coord
                        }); 
                        if drawing {
                            self.drawing = true;
                        }
                        if let (Some(coord), Some(color)) = (coord, drawing_color){
                            let changed = if let Some(prev) = self.prev_coord {
                                self.beads.draw_line(color, prev, coord)
                            } else {
                                self.beads.set_value(color, coord)
                            };
                            if changed {
                                self.undo = 0;
                            }
                            self.prev_coord = Some(coord);
                        }
                    });
                });
        });
    }
}


struct ColorBox<'a> {
    options: &'a Settings,
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
