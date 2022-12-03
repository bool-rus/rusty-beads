use serde::{Serialize, Deserialize};
use super::Coord;

#[derive(Serialize, Deserialize)]
enum SchemaOld {
    FirstOffset,
    SecondOffset,
    Straight,
}
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum SchemaCompat {
    Old(SchemaOld),
    Actual{base_offset: usize, offset_step: usize},
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[serde(from = "SchemaCompat")]
pub struct Schema {
    base_offset: usize,
    offset_step: usize,
}

impl Default for Schema {
    fn default() -> Self {
        Self {base_offset: 2, offset_step: 1}
    }
}

impl From<SchemaCompat> for Schema {
    fn from(value: SchemaCompat) -> Self {
        match value {
            SchemaCompat::Old(SchemaOld::Straight) => Self {base_offset: 1, offset_step: 0},
            SchemaCompat::Old(_) => Self {base_offset: 2, offset_step: 1},
            SchemaCompat::Actual { base_offset, offset_step } => Self {base_offset, offset_step},
        }
    }
}

impl Schema {
    pub fn switch(self) -> Self {
        match self {
            Self {base_offset: 1, offset_step: 0} => Self {base_offset: 4, offset_step: 1},
            Self {base_offset: 4, offset_step: 1} => Self {base_offset: 3, offset_step: 1},
            Self {base_offset: 3, offset_step: 1} => Self {base_offset: 7, offset_step: 3},
            Self {base_offset: 7, offset_step: 3} => Self {base_offset: 2, offset_step: 1},
            Self {base_offset: 2, offset_step: 1} => Self {base_offset: 1, offset_step: 0},
            _ => Self {base_offset: 1, offset_step: 0}
        }
    }
    pub fn calculate_rotation(&self, row: usize, width: usize, rotation: usize) -> usize {
        width - (rotation + row*self.offset_step/self.base_offset) % width
    }
    pub fn calculate_offset(&self, row: usize) -> usize {
        row * self.offset_step % self.base_offset
    }
    pub fn base(&self) -> usize {
        self.base_offset
    }
    pub fn make_line(&self, start: Coord, end: Coord, width: usize) -> Vec<Coord> {
        match self {
            Self {base_offset: 1, offset_step: 0} => make_line(start, end, width),
            _ => make_line_offset(start, end, width),
        }
    }
}

fn normalize_x(dist: &mut f32, width: usize) {
    let width = width as f32;
    if 2.0*dist.abs() > width {
        if *dist > 0.0 {
            *dist -= width;
        } else {
            *dist += width;
        }
    }
}

fn make_line_offset(start: Coord, end: Coord, width: usize) -> Vec<Coord> {
    let coords = make_line(start, end, width);
    let egui::Vec2{x,y} = end - start;
    let need_correction = (x*y).is_sign_negative(); //координаты направлены разные стороны
    let Coord{x: mut px,y: mut py } = start;
    let mut result = Vec::with_capacity(coords.len()*2);
    for Coord{x,y} in coords {
        if need_correction && x != px && y != py {
            result.push(Coord {x, y: py});
        }
        result.push(Coord{x,y});
        (px, py) = (x, y);
    };
    result
}

fn make_line(start: Coord, end: Coord, width: usize) -> Vec<Coord> {
    if start == end {
        return vec![start];
    }
    let startx = (start.x + width) as f32; //добавляем ширину, чтобы не перейти через 0
    let starty = start.y as f32;
    let egui::Vec2 {mut x, y} = end - start;
    normalize_x(&mut x, width);
    let dots = make_dots(x,y);
    dots.into_iter().map(|(x,y)| {
        let y = (y + starty).round() as usize;
        let x = (x + startx).round() as usize % width;
        Coord{x,y}
    }).collect()
}

fn make_dots(mut x: f32, mut y: f32) -> Vec<(f32,f32)> {
    use std::mem::swap;
    //теперь мы будем строить функцию y = a * x
    //но в качестве x нам нужна самая большая координата дистанции
    let transposition = y.abs() > x.abs(); 
    if transposition { // если y > x, значит, меняем их местами (потом поменяем обратно)
        swap(&mut x, &mut y);
    }
    let a = y/x;
    let n = x.abs();
    let mut result = Vec::with_capacity(n.abs() as usize);
    let step = if x > 0.0 { 1.0 } else { -1.0 };
    let mut i = 0.0f32;
    while i.abs() <= n.abs() {
        let mut x = i;
        let mut y = a * x;
        if transposition {
            swap(&mut x, &mut y);
        }
        result.push((x,y));
        i += step;
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    fn assert_line(a: (usize, usize), b: (usize, usize), line: Vec<(usize,usize)>, width: usize) {
        let (x,y) = a;
        let a = Coord {x, y};
        let ib = a;
        let (x,y) = b;
        let b = Coord{x,y};
        let ia = b;
        let line: Vec<_> = line.into_iter().map(|(x,y)|Coord{x,y}).collect();
        let iline: Vec<_> = line.iter().rev().copied().collect();
        assert_eq!(line, make_line(a, b, width));
        assert_eq!(iline, make_line(ia, ib, width));
    }

    #[test]
    fn line_0() {
        let width = 100;
        let a = (94,16);
        let b = (92,16);
        let line = vec![(94,16),(93,16),(92,16)];
        assert_line(a, b, line, width);
    }
    #[test]
    fn line_90() {
        let width = 100;
        let a = (91,12);
        let b = (91,14);
        let line = vec![(91,12), (91,13), (91,14)];
        assert_line(a, b, line, width);
    }
    #[test]
    fn line_45() {
        let width = 100;
        let a = (95,14);
        let b = (93,12);
        let line = vec![(95,14), (94,13), (93,12)];
        assert_line(a, b, line, width);
    }    
    #[test]
    fn line_135() {
        let width = 100;
        let a = (89,12);
        let b = (87,14);
        let line = vec![(89,12), (88,13), (87,14)];
        assert_line(a, b, line, width);
    }
    #[test]
    fn line_26() {
        let width = 100;
        let a = (92,19);
        let b = (89,18);
        let line = vec![(92,19),(91,19),(90,18),(89,18)];
        assert_line(a, b, line, width);
    }
    #[test]
    fn line_154() {
        let width = 100;
        let a = (97,18);
        let b = (94,19);
        let line = vec![(97,18),(96,18),(95,19),(94,19)];
        assert_line(a, b, line, width);
    }
    #[test]
    fn line_37() {
        let width = 100;
        let a = (95,28);
        let b = (92,26);
        let line = vec![(95,28),(94,27),(93,27),(92,26)];
        assert_line(a, b, line, width);
    }    
    #[test]
    fn line_143() {
        let width = 100;
        let a = (95,22);
        let b = (92,24);
        let line = vec![(95,22),(94,23),(93,23),(92,24)];
        assert_line(a, b, line, width);
    }
    #[test]
    fn line_53() {
        let width = 100;
        let a = (90,24);
        let b = (88,21);
        let line = vec![(90,24),(89,23),(89,22),(88,21)];
        assert_line(a, b, line, width);
    }
    #[test]
    fn line_127() {
        let width = 100;
        let a = (86,22);
        let b = (84,25);
        let line = vec![(86,22),(85,23),(85,24),(84,25)];
        assert_line(a, b, line, width);
    }
}