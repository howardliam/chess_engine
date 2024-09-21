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
    pub const WHITE_PAWN: Self = Self {
        kind: PieceKind::Pawn,
        colour: Colour::White,
    };

    pub const WHITE_KNIGHT: Self = Self {
        kind: PieceKind::Knight,
        colour: Colour::White,
    };

    pub const WHITE_BISHOP: Self = Self {
        kind: PieceKind::Bishop,
        colour: Colour::White,
    };

    pub const WHITE_ROOK: Self = Self {
        kind: PieceKind::Rook,
        colour: Colour::White,
    };

    pub const WHITE_QUEEN: Self = Self {
        kind: PieceKind::Queen,
        colour: Colour::White,
    };

    pub const WHITE_KING: Self = Self {
        kind: PieceKind::King,
        colour: Colour::White,
    };

    pub const BLACK_PAWN: Self = Self {
        kind: PieceKind::Pawn,
        colour: Colour::Black,
    };

    pub const BLACK_KNIGHT: Self = Self {
        kind: PieceKind::Knight,
        colour: Colour::Black,
    };

    pub const BLACK_BISHOP: Self = Self {
        kind: PieceKind::Bishop,
        colour: Colour::Black,
    };

    pub const BLACK_ROOK: Self = Self {
        kind: PieceKind::Rook,
        colour: Colour::Black,
    };

    pub const BLACK_QUEEN: Self = Self {
        kind: PieceKind::Queen,
        colour: Colour::Black,
    };

    pub const BLACK_KING: Self = Self {
        kind: PieceKind::King,
        colour: Colour::Black,
    };

    pub fn new(kind: PieceKind, colour: Colour) -> Self {
        Self { kind, colour }
    }
}

impl From<char> for Piece {
    fn from(value: char) -> Self {
        let colour = match value.is_uppercase() {
            true => Colour::White,
            false => Colour::Black,
        };

        let kind = match value.to_uppercase().next().unwrap() {
            'P' => PieceKind::Pawn,
            'N' => PieceKind::Knight,
            'B' => PieceKind::Bishop,
            'R' => PieceKind::Rook,
            'Q' => PieceKind::Queen,
            'K' => PieceKind::King,
            _ => panic!("Bad char for conversion"),
        };

        Self::new(kind, colour)
    }
}

impl ToString for Piece {
    fn to_string(&self) -> String {
        match (self.colour, self.kind) {
            (Colour::White, PieceKind::Pawn) => "♙".to_owned(),
            (Colour::White, PieceKind::Knight) => "♘".to_owned(),
            (Colour::White, PieceKind::Bishop) => "♗".to_owned(),
            (Colour::White, PieceKind::Rook) => "♖".to_owned(),
            (Colour::White, PieceKind::Queen) => "♕".to_owned(),
            (Colour::White, PieceKind::King) => "♔".to_owned(),
            (Colour::Black, PieceKind::Pawn) => "♟".to_owned(),
            (Colour::Black, PieceKind::Knight) => "♞".to_owned(),
            (Colour::Black, PieceKind::Bishop) => "♝".to_owned(),
            (Colour::Black, PieceKind::Rook) => "♜".to_owned(),
            (Colour::Black, PieceKind::Queen) => "♛".to_owned(),
            (Colour::Black, PieceKind::King) => "♚".to_owned(),
        }
    }
}
