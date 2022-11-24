use crate::wrapper::Invertable;

use super::*;

impl BeadsLine<Bead<Color32>> {
    pub fn show_summary(&mut self, ctx: &Context, open: &mut bool ) {
        Window::new("summary").open(open).show(ctx, |ui| {
            ScrollArea::vertical().show(ui, |ui|{
                ui.horizontal_wrapped(|ui|{
                    for (bead, count) in self.line_mut().into_iter() {
                        ui.spacing_mut().item_spacing = vec2(10.0, 5.0);
                        let text = format!("{:^4}", *count);
                        let text_color = bead.color.invert();
                        ui.checkbox(&mut bead.filled, 
                            RichText::new(text).background_color(bead.color).color(text_color).monospace());
                        ui.label("➡");
                    }
                });
            });
        });
    }
}