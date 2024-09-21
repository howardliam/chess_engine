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
