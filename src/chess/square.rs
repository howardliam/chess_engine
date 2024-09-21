use std::{
    fmt,
    ops::{Add, AddAssign, Sub, SubAssign},
};

use regex::Regex;

use super::{coord::Coord, direction::Direction};

#[derive(Debug, Clone, Copy)]
pub struct Square(pub usize);

impl Square {
    pub fn new(square: usize) -> Self {
        Self(square & 0b111111)
    }

    pub fn to_coord(&self) -> Coord {
        let file = (self.0 & 0b111) as i32;
        let rank = ((self.0 >> 3) & 0b111) as i32;

        Coord::new(file, rank)
    }

    pub fn from_en_passant(square: String) -> Option<Self> {
        let regex = Regex::new(r"^[a-h][3-6]$").unwrap();
        if regex.is_match(&square.as_str()) {
            let file_char = square.chars().nth(0).unwrap();
            let rank_char = square.chars().nth(1).unwrap();

            let file = (file_char as i32) - ('a' as i32);
            let rank = (rank_char as i32) - ('1' as i32);

            Some(Self::from(Coord::new(file, rank)))
        } else {
            None
        }
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
        if self.0 > 63 {
            self.0 = 63;
        }
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
        if self.0 > 63 {
            self.0 = 63;
        }
    }
}

impl SubAssign<Direction> for Square {
    fn sub_assign(&mut self, rhs: Direction) {
        self.0 -= rhs as usize;
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let file = self.0 & 0b111;
        let rank = (self.0 >> 3) & 0b111;

        let file_char = (file as u8 + b'a') as char;

        write!(f, "{}{}", file_char, rank)
    }
}

// impl ToString for Square {
//     fn to_string(&self) -> String {
//         let file = self.0 & 0b111;
//         let rank = (self.0 >> 3) & 0b111;

//         let file_char = (file as u8 + b'a') as char;

//         format!("{}{}", file_char, rank)
//     }
// }
