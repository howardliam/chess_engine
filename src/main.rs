use chess::{castling::CastlingRights, colour::Colour, game::GameState};

mod chess;

fn main() {
    let fen = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    let game_state = GameState::from_fen(fen);

    println!("{}", game_state.side_to_move);
}
