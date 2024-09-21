use std::ops::{Add, AddAssign, Sub, SubAssign};

use super::{coord::Coord, direction::Direction};

#[derive(Clone, Copy)]
pub struct Square(pub usize);

impl Square {
    pub fn new(square: usize) -> Self {
        Self(square)
    }

    pub fn to_coord(&self) -> Coord {
        let file = (self.0 & 0b111) as i32;
        let rank = ((self.0 >> 3) & 0b111) as i32;

        Coord::new(file, rank)
    }
}

impl From<Coord> for Square {
    fn from(value: Coord) -> Self {
        let square = (value.rank * 8 + value.file) as usize;

        Self::new(square)
    }
}

impl Add<i32> for Square {
    type Output = Self;

    fn add(self, rhs: i32) -> Self::Output {
        Square::new(self.0 + rhs as usize)
    }
}

impl Sub<i32> for Square {
    type Output = Self;

    fn sub(self, rhs: i32) -> Self::Output {
        Square::new(self.0 - rhs as usize)
    }
}

impl AddAssign<i32> for Square {
    fn add_assign(&mut self, rhs: i32) {
        self.0 += rhs as usize;
    }
}

impl SubAssign<i32> for Square {
    fn sub_assign(&mut self, rhs: i32) {
        self.0 -= rhs as usize;
    }
}

impl Add<Direction> for Square {
    type Output = Self;

    fn add(self, rhs: Direction) -> Self::Output {
        Square::new(self.0 + rhs as usize)
    }
}

impl Sub<Direction> for Square {
    type Output = Self;

    fn sub(self, rhs: Direction) -> Self::Output {
        Square::new(self.0 - rhs as usize)
    }
}

impl AddAssign<Direction> for Square {
    fn add_assign(&mut self, rhs: Direction) {
        self.0 += rhs as usize;
    }
}

impl SubAssign<Direction> for Square {
    fn sub_assign(&mut self, rhs: Direction) {
        self.0 -= rhs as usize;
    }
}
