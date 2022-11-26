use egui::Color32;

use super::*;

struct Action (Color32, Coord);

#[derive(Default)]
pub struct Undo{
    line: BeadsLine<Color32>,
    undo: Vec<Action>,
    redo: Vec<Action>,
}

impl Undo {
    pub fn set_value(&mut self, value: Color32, coord: Coord) {
        if let Some(prev) = self.line.set_value(value, coord) {
            self.undo.push(Action(prev.color, coord));
            self.redo.clear();
            if self.undo.len() % 1000 == 0 {
                println!("уже {}", self.undo.len());
            }
        }
    }
    pub fn line(&self) -> &BeadsLine<Color32> {
        &self.line
    }
    pub fn line_mut(&mut self) -> &mut BeadsLine<Color32> {
        &mut self.line
    }
    pub fn resize(&mut self, size: Size) {
        self.line.resize(size);
        self.undo.clear();
        self.redo.clear();
    }
    pub fn undo(&mut self) {
        if let Some(action) = self.undo.pop() {
            let Action(value, coord) = action;
            if let Some(prev) = self.line.set_value(value, coord) {
                self.redo.push(Action(prev.color, coord));
            }
        }
    }
    pub fn redo(&mut self) {
        use std::mem::swap;
        swap(&mut self.undo, &mut self.redo);
        self.undo();
        swap(&mut self.undo, &mut self.redo);
    }
}

impl From<BeadsLine<Color32>> for Undo {
    fn from(line: BeadsLine<Color32>) -> Self {
        Self {line, undo: vec![], redo: vec![]}
    }
}