use super::{castling::CastlingRights, colour::Colour, fen::Fen, square::Square};

/// Representation of the state of a game of chess, not including the board.
pub struct GameState {
    pub side_to_move: Colour,

    pub white_castling_rights: CastlingRights,
    pub black_castling_rights: CastlingRights,

    pub en_passant_square: Option<Square>,

    pub halfmove_clock: i32,
    pub fullmove_number: i32,
}

impl GameState {
    pub fn from_fen(fen_fragments: Fen) -> Self {
        let side_to_move = Colour::from(fen_fragments.side_to_move);

        let (white_castling_rights, black_castling_rights) =
            CastlingRights::from_fen_part(fen_fragments.castling_rights);

        let en_passant_square = Square::from_en_passant(fen_fragments.en_passant_square);

        let halfmove_clock = fen_fragments.halfmove_clock;
        let fullmove_number = fen_fragments.fullmove_number;

        Self {
            side_to_move,
            white_castling_rights,
            black_castling_rights,
            en_passant_square,
            halfmove_clock,
            fullmove_number,
        }
    }
}

impl Default for GameState {
    /// Based on starting position:
    /// rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1.
    fn default() -> Self {
        Self {
            side_to_move: Colour::White,
            white_castling_rights: CastlingRights::Full,
            black_castling_rights: CastlingRights::Full,
            en_passant_square: None,
            halfmove_clock: 0,
            fullmove_number: 1,
        }
    }
}
