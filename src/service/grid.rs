use std::num::NonZeroUsize;
use core::mem;
use std::sync::Arc;
use crate::model::*;

#[derive(Debug, Clone)]
pub enum Message<T: ColorTrait> {
    Ignore,
    Undo,
    Redo,
    Draw(Coord),
    Grow(Side),
    Shrink(Side),
    Resize(Size),
    Updated(Arc<Model<T>>),
    Loaded(Arc<Model<T>>),
    ToggleLineItem(usize),
    SchemaChange,
    ActivateColor(T),
    AddColor(T),
    RemoveColor,
    DrawColor(Coord, T),
    MoveSeam(isize),
}

pub struct Service<T: ColorTrait> {
    model: Model<T>,
    undo: Vec<Message<T>>,
    redo: Vec<Message<T>>,
}

impl<T: ColorTrait> Service<T> {
    pub fn new(model: Model<T>) -> Self {
        Self {
            model,
            undo: Vec::new(),
            redo: Vec::new(),
        }
    }
    fn updated(&self) -> Message<T> {
        Message::Updated(Arc::new(self.model.clone()))
    }
    fn push_undo(&mut self, msg: Message<T>) {
        self.undo.push(msg);
        self.redo.clear();
    }
}

impl<T: Default + ColorTrait> super::Service for Service<T> {
    type Message = Message<T>;

    fn service(&mut self, msg: Self::Message) -> Result<Option<Self::Message>, String> {
        use Message::*;
        Ok(match msg {
            Draw(Coord{x,y}) => self.model
                    .set(x,y)
                    .map(|prev| {
                        prev.and_then(|Bead{color, ..}|{
                            self.push_undo(DrawColor(Coord{x,y}, color));
                            Some(self.updated())
                        })
                    })?,
            Grow(side) => {
                self.model.grow(side, Default::default());
                self.push_undo(Shrink(side));
                Some(self.updated())
            },
            Shrink(side) => {
                self.model.shrink(side)?;
                self.push_undo(Grow(side));
                Some(self.updated())
            },
            Resize(size) => {
                let prev = self.model.size();
                self.push_undo(Resize(prev));
                self.model.resize(size);
                Some(self.updated())
            },
            ToggleLineItem(index) => {
                self.model.toggle_filled(index)?;
                Some(self.updated())
            },
            SchemaChange => {
                let schema = self.model.schema();
                self.model.set_schema(schema.switch());
                Some(self.updated())
            },
            Undo => {
                let mut undo = Vec::new();
                mem::swap(&mut self.undo, &mut undo);
                mem::swap(&mut self.undo, &mut self.redo);
                let result = match undo.pop() {
                    None => Err("Undo is empty".to_string()),
                    Some(msg) => self.service(msg),
                };
                mem::swap(&mut self.undo, &mut self.redo);
                mem::swap(&mut self.undo, &mut undo);
                result?
            },
            Redo => {
                let mut redo = Vec::new();
                mem::swap(&mut self.redo, &mut redo);
                let result = match redo.pop() {
                    None => Err("Redo is empty".to_string()),
                    Some(msg) => self.service(msg),
                };
                mem::swap(&mut self.redo, &mut redo);
                result?
            },
            Loaded(model) => {
                self.model = model.as_ref().clone();
                Some(Loaded(model))
            },
            AddColor(color) => {
                self.model.add_color(color);
                Some(self.updated())
            },
            RemoveColor => {
                self.model.remove_color();
                Some(self.updated())
            },
            ActivateColor(color) => {
                self.model.activate_color(color);
                Some(self.updated())
            },
            DrawColor(coord, color) => {
                let Coord {x, y} = coord;
                let prev_activated = self.model.activate_color(color);
                self.model.set(x, y)?.map(|Bead {color, ..}|{
                    self.push_undo(DrawColor(coord, color));
                });
                self.model.activate_color(prev_activated);
                Some(self.updated())
            }
            MoveSeam(direction) => {
                self.model.rotate(direction);
                Some(self.updated())
            }
            Updated(_) | Ignore => None,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::service::Service as _;

    fn make() -> Service<u8> {
        let mut model = Model::default();
        model.add_color(1);
        model.add_color(2);
        model.activate_color(3);
        let mut s = Service::new(model);

        for _ in 1..10 {
            s.service(Message::Grow(Side::Top));
            s.service(Message::Grow(Side::Left));
        }
        s
    }

    #[test]
    fn test_undo() {
        let mut s = make();
        s.service(Message::ActivateColor(33));
        s.service(Message::Draw(Coord{ x: 0, y: 0 }));
        s.service(Message::ActivateColor(34));
        s.service(Message::Draw(Coord{ x: 0, y: 0 }));
        s.service(Message::Draw(Coord{ x: 0, y: 0 }));
        s.service(Message::ActivateColor(35));
        s.service(Message::Draw(Coord{ x: 0, y: 0 }));
        let vars: Vec<_> = vec![Message::Undo; 2].into_iter().map(|m|{
           match s.service(m).expect("undo must return message").unwrap() {
               Message::Updated(grid) => grid.as_ref().grid().as_table_iter()
               .nth(0).unwrap()
               .nth(0)
               .map(|Bead{color, ..}|*color)
               .unwrap(),
               _ => {panic!("undo must return updated")},
           }
        }).collect();
        assert_eq!(vars, vec![34,33]);
        let vars: Vec<_> = vec![Message::Redo; 2].into_iter().map(|m|{
            match s.service(m).expect("redo must return message").unwrap() {
                Message::Updated(grid) => grid.as_ref().grid().as_table_iter()
                .nth(0).unwrap()
                .nth(0)
                .map(|Bead{color, ..}|*color)
                .unwrap(),
                _ => {panic!("undo must return updated")},
            }
        }).collect();
        assert_eq!(vars, vec![34,35]);
        s.service(Message::Undo);
        s.service(Message::Grow(Side::Left));
        let response = s.service(Message::Redo);
        match response {
            Err(_) => {},
            _ => {panic!(format!("unexpected response: {:?}", response))},
        }
    }
}