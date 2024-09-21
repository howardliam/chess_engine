use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Clone, Copy)]
pub struct Coord {
    pub file: i32,
    pub rank: i32,
}

impl Coord {
    pub fn new(file: i32, rank: i32) -> Self {
        Self { file, rank }
    }
}

#[derive(Clone, Copy)]
pub struct Square {
    pub square: usize,
}

impl Square {
    pub fn new(square: usize) -> Self {
        Self { square }
    }

    pub fn to_coord(&self) -> Coord {
        let file = (self.square & 0b111) as i32;
        let rank = ((self.square >> 3) & 0b111) as i32;

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
        Square::new(self.square + rhs as usize)
    }
}

impl Sub<i32> for Square {
    type Output = Self;

    fn sub(self, rhs: i32) -> Self::Output {
        Square::new(self.square - rhs as usize)
    }
}

impl AddAssign<i32> for Square {
    fn add_assign(&mut self, rhs: i32) {
        self.square += rhs as usize;
    }
}

impl SubAssign<i32> for Square {
    fn sub_assign(&mut self, rhs: i32) {
        self.square -= rhs as usize;
    }
}
