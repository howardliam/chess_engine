use super::{colour::Colour, piece::Piece};

pub struct Board {
    pub board: [Option<Piece>; 64],
}

impl Board {
    pub fn new() -> Self {
        Self { board: [None; 64] }
    }
}

pub struct Bitboard(u64);
