pub enum Colour {
    White,
    Black,
}

impl Colour {
    pub fn invert(&self) -> Self {
        match self {
            Colour::White => Colour::Black,
            Colour::Black => Colour::White,
        }
    }
}
