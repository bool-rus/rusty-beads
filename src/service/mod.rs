mod grid;

pub trait Service {
    type Message;
    fn service(&mut self, msg: Self::Message) -> Option<Self::Message>;
}

