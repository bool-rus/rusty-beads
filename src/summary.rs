use super::*;

impl BeadsLine<Bead<Color32>> {
    pub fn show_summary(&mut self, ctx: &Context, open: &mut bool ) {
        Window::new("summary").open(open).show(ctx, |ui| {
            ui.horizontal_wrapped(|ui|{
                for (bead, count) in self.line_mut() {
                    let text = format!("⬛ {}", *count);
                    ui.checkbox(&mut bead.filled, RichText::new(text).color(bead.color));
                }
            });
        });
    }
}