use std::fmt;

pub struct InvalidFenError;

impl fmt::Display for InvalidFenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid FEN")
    }
}
