#[derive(Clone, Copy)]
pub struct Coord {
    pub file: i32,
    pub rank: i32,
}

impl Coord {
    pub const FILE_A: i32 = 0;
    pub const FILE_B: i32 = Self::FILE_A + 1;
    pub const FILE_C: i32 = Self::FILE_B + 1;
    pub const FILE_D: i32 = Self::FILE_C + 1;
    pub const FILE_E: i32 = Self::FILE_D + 1;
    pub const FILE_F: i32 = Self::FILE_E + 1;
    pub const FILE_G: i32 = Self::FILE_F + 1;
    pub const FILE_H: i32 = Self::FILE_G + 1;

    pub const RANK_1: i32 = 0;
    pub const RANK_2: i32 = Self::RANK_1 + 1;
    pub const RANK_3: i32 = Self::RANK_2 + 1;
    pub const RANK_4: i32 = Self::RANK_3 + 1;
    pub const RANK_5: i32 = Self::RANK_4 + 1;
    pub const RANK_6: i32 = Self::RANK_5 + 1;
    pub const RANK_7: i32 = Self::RANK_6 + 1;
    pub const RANK_8: i32 = Self::RANK_7 + 1;

    pub fn new(file: i32, rank: i32) -> Self {
        Self { file, rank }
    }
}
