use std::hash::{Hasher, Hash};

pub struct Wrapped<T>(T);
pub trait Wrappable {
    fn wrap(&self)->Wrapped<&Self>;
}
impl<T> Wrappable for T {
    fn wrap(&self) -> Wrapped<&Self> {
        Wrapped(self)
    }
}
impl Hash for Wrapped<&iced::Color> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.into_linear().iter().for_each(|f|{
            state.write(&f.to_be_bytes())
        });
    }
}