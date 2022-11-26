use eframe::epaint::ahash::HashMap;

use crate::wrapper::Invertable;

use super::*;
const CHARS: &str = "ABCDEFGHIJKLMNOPQRSTYVWXYZ";

impl BeadsLine<Color32> {
    pub fn show_summary(&mut self, ctx: &Context, open: &mut bool ) {
        Window::new("summary").open(open).show(ctx, |ui| {
            ScrollArea::vertical().show(ui, |ui|{
                ui.horizontal_wrapped(|ui|{
                    let map: indexmap::IndexMap<_,_> = self.summary().into_iter().zip(CHARS.chars())
                        .map(|((k,v), c)|(k, (v, c))).collect();
                    for (k,(v,c)) in &map {
                        ui.label(format!("{c}: "));
                        ui.label(RichText::new("⬛").color(*k));
                        ui.label(format!(" {v}"));
                        ui.end_row();
                    }
                    ui.vertical(|ui|ui.separator());
                    let map: HashMap<_,_> = map.into_iter().map(|(k,(_,c))|(k,c)).collect();
                    for (bead, count) in self.line_mut().into_iter() {
                        ui.spacing_mut().item_spacing = vec2(10.0, 5.0);
                        let text = format!("{}: {:^4}", map.get(&bead.color).copied().unwrap_or('?'), *count);
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