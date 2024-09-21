use crate::chess::fen::fen_is_valid;

use super::{board::Board, chess_move::ChessMove, colour::Colour, game::GameState};

pub struct GameManager {
    pub player_side: Colour,
    pub engine_side: Colour,

    pub board: Board,
    pub game_state: GameState,
}

impl GameManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn apply_fen(&mut self, fen: String) {
        if !fen_is_valid(fen) {
            panic!("Invalid FEN");
        }

        todo!()
    }

    pub fn dump_fen(&self) -> String {
        todo!()
    }

    pub fn make_move(chess_move: ChessMove) {}
}

impl Default for GameManager {
    fn default() -> Self {
        Self {
            player_side: Colour::White,
            engine_side: Colour::Black,
            board: Board::default(),
            game_state: GameState::default(),
        }
    }
}
