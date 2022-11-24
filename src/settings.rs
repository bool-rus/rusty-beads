use egui::*;
use crate::*;


pub struct DrawOptions {
    pub size: Vec2,
    pub stroke: Stroke,
    pub rounding: Rounding,
    pub filled_stroke: Stroke,
    pub seam_rounding: Rounding,
    factor: f32,
    origin_size: Vec2,
}

impl Default for DrawOptions {
    fn default() -> Self {
        Self { 
            size: vec2(10.0, 10.0), 
            stroke: Stroke::new(0.4, Color32::WHITE), 
            filled_stroke: Stroke::new(0.4, Color32::TRANSPARENT), 
            rounding: Default::default(), 
            seam_rounding: Rounding::same(5.0),
            factor: 1.0,
            origin_size: vec2(10.0, 10.0), 
        }
    }
}

impl DrawOptions {
    pub fn show(&mut self, ctx: &Context, open: &mut bool) {
        Window::new("settings").open(open).show(ctx, |ui| {
            ui.add(Slider::new(&mut self.origin_size.x, 10.0..=100.0).text("↔"));
            ui.add(Slider::new(&mut self.origin_size.y, 10.0..=100.0).text("↕"));
            ui.add(Slider::new(&mut self.factor, 0.1..=10.0).text("×").logarithmic(true));
            self.size = self.origin_size * self.factor;
            ui.separator();
            ui.add(Slider::new(&mut self.stroke.width, 0.0..=5.0).text("☐"));
            ui.horizontal(|ui| {
                ui.color_edit_button_srgba(&mut self.stroke.color);
                ui.label("border color");
            });
            ui.separator();
            ui.add(Slider::new(&mut self.filled_stroke.width, 0.0..=5.0).text("⛶"));
            ui.horizontal(|ui| {
                ui.color_edit_button_srgba(&mut self.filled_stroke.color);
                ui.label("filled border color");
            });
            ui.separator();
        });
    }
}
