#[derive(Clone, Copy)]
pub struct Square {
    pub file: u8,
    pub rank: u8,
}

impl Square {
    pub fn new(file: u8, rank: u8) -> Self {
        Self { file, rank }
    }

    pub fn to_index(&self) -> usize {
        ((self.rank * 8) + self.file) as usize
    }
}

impl From<usize> for Square {
    fn from(value: usize) -> Self {
        let file = (value & 0b111) as u8;
        let rank = ((value >> 3) & 0b111) as u8;

        Self::new(file, rank)
    }
}
