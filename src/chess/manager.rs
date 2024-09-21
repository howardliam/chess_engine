use crate::chess::fen::{fen_is_valid, Fen};

use super::{board::Board, chess_move::ChessMove, colour::Colour, game_state::GameState};

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
        let fen_fragments = match Fen::try_from(fen_string) {
            Ok(fen) => fen,
            Err(err) => panic!("{err}"),
        };

        self.board = Board::from_fen(&fen_fragments.board_layout);
        self.game_state = GameState::from_fen(fen_fragments);
    }

    pub fn dump_fen(&self) -> String {
        todo!()
    }

    pub fn make_move(&self, chess_move: ChessMove) {}

    pub fn print(&self) {
        let GameState {
            side_to_move,
            white_castling_rights,
            black_castling_rights,
            en_passant_square,
            halfmove_clock,
            fullmove_number,
        } = &self.game_state;

        let wcr = white_castling_rights.to_string_colour(Colour::White);
        let bcr = black_castling_rights.to_string_colour(Colour::Black);

        println!("{side_to_move}'s turn | {wcr}{bcr}");
        println!("{}", self.board);
    }
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
