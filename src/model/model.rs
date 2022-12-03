use egui::Color32;

use super::*;

#[derive(Default)]
struct Action (Color32, Coord);

pub struct Model {
    line: BeadsLine<Color32>,
    height: usize,
    undo: Vec<Action>,
    redo: Vec<Action>,
}

impl Default for Model {
    fn default() -> Self {
        let line: BeadsLine<_> = Default::default();
        Self { height: line.calculate_height(), line, undo: Default::default(), redo: Default::default() }
    }
}

impl Model {
    pub fn draw_line(&mut self, color: Color32, start: Coord, end: Coord) -> bool {
        let line = self.line.schema.make_line(start, end, self.line.width);
        let mut changed = false;
        for coord in line {
            if self.set_value(color, coord) {
                changed = true;
            }
        };
        changed
    }
    pub fn set_value(&mut self, value: Color32, coord: Coord) -> bool {
        if let Some(prev) = self.line.set_value(value, coord) {
            self.undo.push(Action(prev.color, coord));
            self.redo.clear();
            if self.undo.len() % 1000 == 0 {
                println!("уже {}", self.undo.len());
            }
            true
        } else {
            false
        }
    }
    pub fn height(&self) -> usize {
        self.height
    } 
    pub fn line(&self) -> &BeadsLine<Color32> {
        &self.line
    }
    pub fn line_mut(&mut self) -> &mut BeadsLine<Color32> {
        &mut self.line
    }
    pub fn resize(&mut self, size: Size) {
        self.line.resize(size);
        self.height = size.height.get();
        self.undo.clear();
        self.redo.clear();
    }
    pub fn undo_at(&mut self, n: usize) {
        let target = n as isize;
        let current = self.redo.len() as isize;
        let delta = target - current;
        if delta >= 0 {
            self.undo_n(delta as usize);
        } else {
            self.redo_n(delta.abs() as usize);
        }
    }
    pub fn max_undo(&self) -> usize {
        self.undo.len() + self.redo.len()
    }
    fn undo_n(&mut self, n: usize) {
        for _ in 0..n {
            if let Some(action) = self.undo.pop() {
                let Action(value, coord) = action;
                if let Some(prev) = self.line.set_value(value, coord) {
                    self.redo.push(Action(prev.color, coord));
                }
            }
        }
    }
    fn redo_n(&mut self, n: usize) {
        use std::mem::swap;
        swap(&mut self.undo, &mut self.redo);
        self.undo_n(n);
        swap(&mut self.undo, &mut self.redo);
    }
    pub fn undo(&mut self) {
        self.undo_n(1);
    }
    pub fn redo(&mut self) {
        self.redo_n(1);
    }
}

impl From<BeadsLine<Color32>> for Model {
    fn from(line: BeadsLine<Color32>) -> Self {
        let height = line.calculate_height();
        Self {line, undo: vec![], redo: vec![], height}
    }
}