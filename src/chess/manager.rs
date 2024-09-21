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
