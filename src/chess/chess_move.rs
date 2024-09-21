use super::square::Square;

const START_SQUARE_MASK: u32 = 0b000000_111111;
const TARGET_SQUARE_MASK: u32 = 0b111111_000000;
const SQUARE_MASK: u32 = 0b111111;

/// A chess move
#[derive(Clone, Copy)]
pub struct ChessMove(u32);

impl ChessMove {
    pub fn new(start_square: Square, target_square: Square) -> Self {
        let start = start_square.to_index() as u32 & SQUARE_MASK;
        let target = target_square.to_index() as u32 & SQUARE_MASK;

        let move_integer = start | (target << 6);
        return Self(move_integer);
    }

    pub fn get_start_square(&self) -> Square {
        let square = (self.0 & START_SQUARE_MASK) as usize;
        return Square::from(square);
    }

    pub fn get_target_square(&self) -> Square {
        let square = (self.0 & TARGET_SQUARE_MASK) as usize;
        return Square::from(square);
    }
}
