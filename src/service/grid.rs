use crate::entities::*;
use crate::grid::Grid;
use std::rc::Rc;
use std::num::NonZeroU32;
use std::fmt::Debug;
use std::thread::current;

#[derive(Debug, Clone)]
pub enum Message<T: Debug + Clone> {
    Ignore,
    Point(Coord),
    Grow(Side),
    Shrink(Side),
    Resize(Size),
    Updated(Rc<Grid<T>>),
    Err(String), //TODO: вместо строки надо бы какой-нибудь тип
}

impl<T: Debug + Clone> From<Result<Message<T>, String>> for Message<T> {
    fn from(result: Result<Message<T>, String>) -> Self {
        match result {
            Ok(msg) => msg,
            Err(e) => Message::Err(e),
        }
    }
}

pub struct Service<T: Debug + Clone> {
    grid: Grid<T>,
    current: T,
}

impl<T: Debug + Clone> Service<T> {
    fn updated(&self) -> Message<T> {
        Message::Updated(Rc::new(self.grid.clone()))
    }
}

impl<T: Default + Debug + Clone> super::Service for Service<T> {
    type Message = Message<T>;

    fn service(&mut self, msg: Self::Message) -> Option<Self::Message> {
        use Message::*;
        //TODO: implement undo logic
        match msg {
            Point(Coord{x,y}) => {
                let msg = self.grid
                    .set(x,y, self.current.clone())
                    .map(|prev| {
                        self.updated()
                    });
                Some(msg.into())
            },
            Grow(side) => {
                self.grid.grow(side, Default::default());
                Some(self.updated())
            },
            Shrink(side) => {
                let msg = self.grid.shrink(side)
                    .map(|_| {
                        self.updated()
                    });
                Some(msg.into())
            },
            Resize(Size { width, height }) => {
                self.grid.resize(width, height);
                Some(self.updated())
            },
            Updated(_)| Err(_) | Ignore => None,
        }
    }
}