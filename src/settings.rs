use egui::*;
use crate::*;


pub struct Settings {
    pub size: Vec2,
    pub stroke: Stroke,
    pub rounding: Rounding,
    pub filled_stroke: Stroke,
    pub seam_rounding: Rounding,
    factor: f32,
    origin_size: Vec2,
    width: String,
    height: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self { 
            size: vec2(10.0, 10.0), 
            stroke: Stroke::new(0.4, Color32::WHITE), 
            filled_stroke: Stroke::new(0.4, Color32::TRANSPARENT), 
            rounding: Default::default(), 
            seam_rounding: Rounding::same(5.0),
            factor: 1.0,
            origin_size: vec2(10.0, 10.0), 
            width: 100.to_string(),
            height: 100.to_string(),
        }
    }
}

impl Settings {
    pub fn show(&mut self, ctx: &Context, open: &mut bool, beads: &mut Undo) {
        Window::new("settings").open(open).show(ctx, |ui| {
            self.show_ui(ui, beads);
        });
    }
    fn show_ui(&mut self, ui: &mut Ui, beads: &mut Undo) {
        ScrollArea::vertical().show(ui, |ui| {
            ui.add(Slider::new(&mut self.origin_size.x, 10.0..=100.0).text("↔"));
            ui.add(Slider::new(&mut self.origin_size.y, 10.0..=100.0).text("↕"));
            ui.add(Slider::new(&mut self.factor, 0.1..=10.0).text("🇽").logarithmic(true));
            self.size = self.origin_size * self.factor;
            ui.separator();
            ui.add(Slider::new(&mut self.stroke.width, 0.0..=5.0).text("☐"));
            ui.horizontal(|ui| {
                ui.color_edit_button_srgba(&mut self.stroke.color);
                ui.label(text4btn("☐").color(self.stroke.color));
            });
            ui.separator();
            ui.add(Slider::new(&mut self.filled_stroke.width, 0.0..=5.0).text("⛶"));
            ui.horizontal(|ui| {
                ui.color_edit_button_srgba(&mut self.filled_stroke.color);
                ui.label(text4btn("⛶").color(self.filled_stroke.color));
            });
            ui.separator();

            ui.horizontal_wrapped(|ui| {
                ui.text_edit_singleline(&mut self.width);
                ui.label("↔");
                ui.end_row();
                ui.text_edit_singleline(&mut self.height);
                ui.label("↕");
                ui.end_row();
                if ui.button("OK").clicked() {
                    match (self.width.parse(), self.height.parse()) {
                        (Ok(width), Ok(height)) => {
                            beads.resize(Size {width, height});
                        }
                        _ => {},
                    };
                }
                ui.end_row();
                ui.vertical(|ui|ui.separator());
                if ui.button(text4btn("◀")).clicked() {
                    beads.line_mut().rotate(-1);
                };
                ui.label(text4btn("💫"));
                if ui.button(text4btn("▶")).clicked() {
                    beads.line_mut().rotate(1);
                };
                ui.end_row();
                ui.vertical(|ui|ui.separator());
                if ui.button("сменить схему").clicked() {
                    let schema = beads.line_mut().schema.switch();
                    beads.line_mut().change_schema(schema);
                }
            });
        });
    }
}
