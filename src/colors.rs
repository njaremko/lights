use std::fmt;
use std::slice::Iter;
use self::Color::*;

#[allow(dead_code)]
#[derive(Debug)]
pub enum Color {
    Blue,
    Cyan,
    Green,
    Red,
}

impl Color {
    pub fn value(&self) -> (u16, u8) {
        match *self {
            Blue => (46920, 254),
            Cyan => (32767, 254),
            Green => (25500, 254),
            Red => (0, 254),
        }
    }

    pub fn iterator() -> Iter<'static, Color> {
        static COLORS: [Color; 4] = [Blue, Cyan, Green, Red];
        COLORS.into_iter()
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
