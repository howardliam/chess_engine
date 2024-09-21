use super::{castling::CastlingRights, colour::Colour, square::Square};

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
    pub fn from_fen(fen: String) -> Self {
        let parts = fen.split(' ').collect::<Vec<&str>>();

        let side_to_move = match parts[1] {
            "w" => Colour::White,
            "b" => Colour::Black,
            _ => panic!("Bad FEN"),
        };

        let (white_castling_rights, black_castling_rights) =
            CastlingRights::from_fen_part(parts[2].to_owned());

        let en_passant_square = None;

        let halfmove_clock = parts[4].parse::<i32>().unwrap_or_default();
        let fullmove_number = parts[5].parse::<i32>().unwrap_or_default();

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
