
impl Into<iced::Color> for Color {
    fn into(self) -> iced::Color {
        let Self {r,g,b} = self;
        iced::Color::from_rgb8(r,g,b)
    }
}

impl From<iced::Color> for Color {
    fn from(color: iced::Color) -> Self {
        let max = u8::MAX as f32;
        Self {
            r: (max * color.r) as u8,
            g: (max * color.g) as u8,
            b: (max * color.b) as u8,
        }
    }
}

impl Hash for Wrapped<&iced::Color> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.into_linear().iter().for_each(|f|{
            state.write(&f.to_be_bytes())
        });
    }
}