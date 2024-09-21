use crate::chess::fen::{fen_is_valid, Fen};

use super::{board::Board, chess_move::ChessMove, colour::Colour, game::GameState};

pub struct GameManager {
    pub player_side: Colour,
    pub engine_side: Colour,

    pub board: Board,
    pub game_state: GameState,
}

impl GameManager {
    pub fn new(player_side: Colour, engine_side: Colour) -> Self {
        Self {
            player_side,
            engine_side,
            board: Board::default(),
            game_state: GameState::default(),
        }
    }

    pub fn apply_fen(&mut self, fen_string: String) {
        let fen_fragments = match Fen::from(fen_string) {
            Ok(fen) => fen,
            Err(err) => panic!("{err}"),
        };

        self.board = Board::from_fen(fen_fragments.board_layout);
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
