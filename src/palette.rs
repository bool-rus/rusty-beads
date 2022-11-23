use egui::*;
use fxhash::FxHashSet;

pub struct Palette {
    colors: FxHashSet<Color32>,
    active_color: Color32,
    choise_color: Color32,
}

impl Default for Palette {
    fn default() -> Self {
        use Color32 as C;
        let mut colors = FxHashSet::default();
        for c in [C::YELLOW, C::WHITE, C::BLACK, C::RED, C::BLUE, C::GREEN] {
            colors.insert(c);
        }
        Self { 
            colors, 
            active_color: C::BLUE, 
            choise_color: C::DARK_RED, 
        }
    }
}

impl Palette {
    pub fn active_color(&self) -> Color32 {
        self.active_color
    }
    pub fn show(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui|{
            ui.horizontal_wrapped(|ui| {
                ui.set_max_width(ui.available_width() - 100.0);
                for color in self.colors.clone() {
                    ui.selectable_value(
                        &mut self.active_color,
                        color, 
                        RichText::new("⬛").color(color)
                    );
                }
            });
            if ui.button("➕").clicked() {
                self.colors.insert(self.choise_color);
            }
            ui.color_edit_button_srgba(&mut self.choise_color);
            if ui.button("🗙").clicked() {
                self.colors.remove(&self.active_color);
            }
        });
    }
}