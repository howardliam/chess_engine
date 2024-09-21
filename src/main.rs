use chess::{fen::Fen, manager::GameManager};

mod chess;

fn main() {
    let mut manager = GameManager::default();
    manager.apply_fen(Fen::STARTING_POSITION.to_owned());
    println!("{}", manager.board);
}
