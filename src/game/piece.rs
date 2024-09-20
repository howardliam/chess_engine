use super::colour::Colour;

pub enum PieceKind {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

pub struct Piece {
    pub kind: PieceKind,
    pub colour: Colour,
}

impl Piece {
    pub fn new(kind: PieceKind, colour: Colour) -> Self {
        Self { kind, colour }
    }
}
