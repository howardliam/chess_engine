use chess::{fen::Fen, manager::GameManager};

mod chess;
mod errors;

fn main() {
    let mut manager = GameManager::default();
    manager.apply_fen(Fen::STARTING_POSITION.to_owned());

    manager.print();
}
