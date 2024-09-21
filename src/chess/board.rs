use std::fmt::Display;

use super::{colour::Colour, piece::Piece};

pub struct Board {
    pub board: [Option<Piece>; 64],
}

impl Board {
    pub fn new() -> Self {
        Self { board: [None; 64] }
    }
}

impl Default for Board {
    fn default() -> Self {
        let board = [
            Some(Piece::WHITE_ROOK),
            Some(Piece::WHITE_KNIGHT),
            Some(Piece::WHITE_BISHOP),
            Some(Piece::WHITE_QUEEN),
            Some(Piece::WHITE_KING),
            Some(Piece::WHITE_BISHOP),
            Some(Piece::WHITE_KNIGHT),
            Some(Piece::WHITE_ROOK),
            Some(Piece::WHITE_PAWN),
            Some(Piece::WHITE_PAWN),
            Some(Piece::WHITE_PAWN),
            Some(Piece::WHITE_PAWN),
            Some(Piece::WHITE_PAWN),
            Some(Piece::WHITE_PAWN),
            Some(Piece::WHITE_PAWN),
            Some(Piece::WHITE_PAWN),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(Piece::BLACK_PAWN),
            Some(Piece::BLACK_PAWN),
            Some(Piece::BLACK_PAWN),
            Some(Piece::BLACK_PAWN),
            Some(Piece::BLACK_PAWN),
            Some(Piece::BLACK_PAWN),
            Some(Piece::BLACK_PAWN),
            Some(Piece::BLACK_PAWN),
            Some(Piece::BLACK_ROOK),
            Some(Piece::BLACK_KNIGHT),
            Some(Piece::BLACK_BISHOP),
            Some(Piece::BLACK_QUEEN),
            Some(Piece::BLACK_KING),
            Some(Piece::BLACK_BISHOP),
            Some(Piece::BLACK_KNIGHT),
            Some(Piece::BLACK_ROOK),
        ];

        Self { board }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string = String::new();

        for (i, piece_option) in self.board.iter().enumerate() {
            if i % 8 == 0 {
                string.push('\n');
            }
            let piece_glyph = match piece_option {
                Some(piece) => piece.to_string(),
                None => " ".to_owned(),
            };
            string.push_str(format!(" {}", piece_glyph).as_str());
        }

        write!(f, "{}", string)
    }
}

pub struct Bitboard(u64);
