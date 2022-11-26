use egui::*;
use indexmap::IndexSet;

pub struct Palette {
    colors: IndexSet<Color32>,
    active_color: Color32,
}

impl Default for Palette {
    fn default() -> Self {
        use Color32 as C;
        let mut colors = IndexSet::default();
        for c in [C::YELLOW, C::WHITE, C::BLACK, C::RED, C::BLUE, C::GREEN] {
            colors.insert(c);
        }
        Self { 
            colors, 
            active_color: C::BLUE, 
        }
    }
}

impl Palette {
    pub fn set_colors(&mut self, colors: IndexSet<Color32>) {
        self.colors = colors;
    }
    pub fn active_color(&self) -> Color32 {
        self.active_color
    }
    pub fn show(&mut self, ui: &mut Ui) {
        ui.horizontal_centered(|ui|{
            if ui.button(RichText::new("🗙").size(20.).color(Color32::RED)).clicked() {
                self.colors.remove(&self.active_color);
            }
            ui.color_edit_button_srgba(&mut self.active_color);
            if ui.button(RichText::new("➕").size(20.).color(Color32::GREEN)).clicked() {
                self.colors.insert(self.active_color);
                self.active_color = self.active_color;
            }
            ui.horizontal_wrapped(|ui| {
                for color in self.colors.clone() {
                    ui.selectable_value(
                        &mut self.active_color,
                        color, 
                        RichText::new("⬛").color(color)
                    );
                }
            });
        });
    }
}