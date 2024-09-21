use super::colour::Colour;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PieceKind {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Piece {
    pub kind: PieceKind,
    pub colour: Colour,
}

impl Piece {
    pub fn new(kind: PieceKind, colour: Colour) -> Self {
        Self { kind, colour }
    }
}
