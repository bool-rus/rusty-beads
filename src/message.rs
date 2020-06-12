use crate::entities::StandartMessage;

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Standart(StandartMessage)
}

impl From<StandartMessage> for Message {
    fn from(m: StandartMessage) -> Self {
        Message::Standart(m)
    }
}