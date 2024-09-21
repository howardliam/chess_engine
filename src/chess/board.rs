use std::fmt;

use super::{colour::Colour, piece::Piece};

pub struct Board {
    pub board: [Option<Piece>; 64],
}

impl Board {
    pub fn new() -> Self {
        Self { board: [None; 64] }
    }

    /// Parses the board layout fragment of FEN into board layout
    pub fn from_fen(fen_layout: String) -> Self {
        let mut board: [Option<Piece>; 64] = [None; 64];

        let rank = 0;
        let file = 7;
        for ch in fen_layout.chars() {}

        todo!()
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

impl fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output: Vec<String> = Vec::new();
        output.push("  A B C D E F G H".to_owned());

        let board_chunks = self.board.chunks(8);
        for (i, chunk) in board_chunks.enumerate() {
            let mut row = String::from(format!("{}", i % 8 + 1));
            for piece_option in chunk {
                let piece_glyph = match piece_option {
                    Some(piece) => piece.to_string(),
                    None => " ".to_owned(),
                };

                row.push_str(format!(" {}", piece_glyph).as_str());
            }
            output.push(row);
        }
        output.reverse();

        write!(f, "{}", output.join("\n"))
    }
}

pub struct Bitboard(u64);
