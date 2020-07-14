use crate::entities::*;
use crate::grid::Grid;
use std::rc::Rc;
use std::fmt::Debug;
use std::collections::VecDeque;
use std::num::NonZeroUsize;
use core::mem;
use std::sync::Arc;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum Message<T: Debug + Clone> {
    Ignore,
    Undo,
    Redo,
    Point(Coord, T),
    Grow(Side),
    Shrink(Side),
    Resize(Size),
    Updated(Arc<Grid<T>>),
    Loaded(Grid<T>),
}

#[derive(Default)]
pub struct Service<T: Debug + Clone> {
    grid: Grid<T>,
    undo: Vec<Message<T>>,
    redo: Vec<Message<T>>,
}

impl<T: Debug + Clone> Service<T> {
    fn updated(&self) -> Message<T> {
        Message::Updated(Arc::new(self.grid.clone()))
    }
    fn push_undo(&mut self, msg: Message<T>) {
        self.undo.push(msg);
        self.redo.clear();
    }
}

impl<T: Default + Debug + Clone + PartialEq> super::Service for Service<T> {
    type Message = Message<T>;

    fn service(&mut self, msg: Self::Message) -> Result<Option<Self::Message>, String> {
        use Message::*;
        Ok(match msg {
            Point(Coord{x,y}, new) => {
                let msg = self.grid
                    .set(x,y, new.clone())
                    .map(|prev| {
                        if new != prev {
                            self.push_undo(Point(Coord{x,y},prev));
                        }
                        self.updated()
                    })?;
                Some(msg.into())
            },
            Grow(side) => {
                self.grid.grow(side, Default::default());
                self.push_undo(Shrink(side));
                Some(self.updated())
            },
            Shrink(side) => {
                self.grid.shrink(side)?;
                self.push_undo(Grow(side));
                Some(self.updated())
            },
            Resize(Size { width, height }) => {
                let prev = Size {
                    width: NonZeroUsize::new(self.grid.width()).unwrap(),
                    height: NonZeroUsize::new(self.grid.height()).unwrap(),
                };
                self.push_undo(Resize(prev));
                self.grid.resize(width, height);
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
            Loaded(grid) => {
                self.grid = grid.clone();
                Some(Updated(Arc::new(grid)))
            },
            Updated(_) | Ignore => None,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::service::Service as _;

    fn make() -> Service<u8> {
        let mut s = Service::default();
        for _ in 1..10 {
            s.service(Message::Grow(Side::Top));
            s.service(Message::Grow(Side::Left));
        }
        s
    }

    #[test]
    fn test_undo() {
        let mut s = make();
        s.service(Message::Point(Coord{ x: 0, y: 0 }, 33));
        s.service(Message::Point(Coord{ x: 0, y: 0 }, 34));
        s.service(Message::Point(Coord{ x: 0, y: 0 }, 34));
        s.service(Message::Point(Coord{ x: 0, y: 0 }, 35));
        let vars: Vec<_> = vec![Message::Undo; 2].into_iter().map(|m|{
           match s.service(m).expect("undo must return message") {
               Message::Updated(grid) => {grid.as_table()[0][0]},
               _ => {panic!("undo must return updated")},
           }
        }).collect();
        assert_eq!(vars, vec![34,33]);
        let vars: Vec<_> = vec![Message::Redo; 2].into_iter().map(|m|{
            match s.service(m).expect("redo must return message") {
                Message::Updated(grid) => {grid.as_table()[0][0]},
                _ => {panic!("undo must return updated")},
            }
        }).collect();
        assert_eq!(vars, vec![34,35]);
        s.service(Message::Undo);
        s.service(Message::Grow(Side::Left));
        let response = s.service(Message::Redo);
        match response {
            Some(Message::Error(_)) => {},
            _ => {panic!(format!("unexpected response: {:?}", response))},
        }
    }
}