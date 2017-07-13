#[allow(dead_code)]
pub enum Color {
    BLUE,
    CYAN,
    GREEN,
    RED
}

impl Color {
    pub fn value(&self) -> (u16, u8) {
        match *self {
            Color::BLUE => (46920, 254),
            Color::CYAN => (32767, 254),
            Color::GREEN => (25500, 254),
            Color::RED => (0, 254),
        }
    }
}
