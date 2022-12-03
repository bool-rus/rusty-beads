use serde::{Serialize, Deserialize};


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
}

impl Default for Schema {
    fn default() -> Self {
        Self {base_offset: 2, offset_step: 1}
    }
}
