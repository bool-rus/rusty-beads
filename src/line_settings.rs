use std::{convert::TryInto, num::NonZeroUsize};

use super::*;

pub struct LineSettings {
    width: String,
    height: String,
}

impl Default for LineSettings {
    fn default() -> Self {
        Self { 
            width: 100.to_string(), 
            height: 100.to_string(), 
        }
    }
}

impl LineSettings {
    pub fn show_settings(&mut self, ctx: &Context, open: &mut bool, line: &mut BeadsLine<Color32>) {
        Window::new("bead settings").open(open).show(ctx, |ui|{
            ui.horizontal_wrapped(|ui| {
                ui.text_edit_singleline(&mut self.width);
                ui.label("↔");
                ui.end_row();
                ui.text_edit_singleline(&mut self.height);
                ui.label("↕");
                ui.end_row();
                if ui.button("OK").clicked() {
                    let size = match (self.width.parse(), self.height.parse()) {
                        (Ok(width), Ok(height)) => {
                            if let (Some(width), Some(height)) = (NonZeroUsize::new(width), NonZeroUsize::new(height)) {
                                Some(Size{width, height})
                            } else {
                                None
                            }
                        }
                        _ => None,
                    };
                    if let Some(size) = size {
                        line.resize(size);
                    } else {
                        println!("wrong number");
                    }
                }
            });
        });
    }
}