use super::square::Square;

/// A chess move
#[derive(Clone, Copy)]
pub struct ChessMove(u32);

impl ChessMove {
    pub const START_SQUARE_MASK: u32 = 0b000000_111111;
    pub const TARGET_SQUARE_MASK: u32 = 0b111111_000000;
    pub const SQUARE_MASK: u32 = 0b111111;

    pub fn new(start_square: Square, target_square: Square) -> Self {
        let start = start_square.0 as u32 & Self::SQUARE_MASK;
        let target = target_square.0 as u32 & Self::SQUARE_MASK;

        let move_integer = start | (target << 6);
        return Self(move_integer);
    }

    pub fn get_start_square(&self) -> Square {
        let square = (self.0 & Self::START_SQUARE_MASK) as usize;
        return Square::new(square);
    }

    pub fn get_target_square(&self) -> Square {
        let square = (self.0 & Self::TARGET_SQUARE_MASK) as usize;
        return Square::new(square);
    }
}
