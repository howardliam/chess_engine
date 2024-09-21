use super::{board::Board, colour::Colour, game::GameState};

pub struct GameManager {
    pub player_side: Colour,
    pub engine_side: Colour,

    pub board: Board,
    pub game_state: GameState,
}
