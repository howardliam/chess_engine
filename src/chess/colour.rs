use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Colour {
    White,
    Black,
}

impl From<String> for Colour {
    /// Assumes w or b, panics otherwise.
    fn from(value: String) -> Self {
        match value.as_str() {
            "w" => Self::White,
            "b" => Self::Black,
            _ => panic!("This should be inaccessible"),
        }
    }
}

impl Colour {
    pub fn invert(&self) -> Self {
        match self {
            Colour::White => Colour::Black,
            Colour::Black => Colour::White,
        }
    }
}

impl fmt::Display for Colour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            Colour::White => "White".to_owned(),
            Colour::Black => "Black".to_owned(),
        };

        write!(f, "{}", string)
    }
}
