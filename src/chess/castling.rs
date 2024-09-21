use super::colour::Colour;

pub enum CastlingRights {
    None,
    KingSideOnly,
    QueenSideOnly,
    Full,
}

impl CastlingRights {
    /// Returns tuple of castling rights; white then black.
    pub fn from_fen_part(part: String) -> (Self, Self) {
        if part == "-" {
            return (Self::None, Self::None);
        }

        let mut white_part = String::new();
        let mut black_part = String::new();

        for ch in part.chars() {
            if ch.is_uppercase() {
                white_part.push(ch);
            } else {
                black_part.push(ch);
            }
        }

        (Self::from(white_part), Self::from(black_part))
    }

    pub fn to_fen_part(white: Self, black: Self) -> String {
        let white_part = white.to_string();
        let black_part = black.to_string();

        let part = white_part + black_part.as_str();

        if part.len() == 0 {
            return "-".to_owned();
        }

        part
    }

    pub fn to_string_colour(&self, colour: Colour) -> String {
        let string = self.to_string();
        match colour {
            Colour::White => string,
            Colour::Black => string.to_lowercase(),
        }
    }
}

impl ToString for CastlingRights {
    /// Converts enum into FEN part.
    fn to_string(&self) -> String {
        match self {
            CastlingRights::None => "".to_owned(),
            CastlingRights::KingSideOnly => "K".to_owned(),
            CastlingRights::QueenSideOnly => "Q".to_owned(),
            CastlingRights::Full => "KQ".to_owned(),
        }
    }
}

impl From<String> for CastlingRights {
    /// Converts from one side FEN part into enum.
    fn from(value: String) -> Self {
        match value.to_uppercase().as_str() {
            "KQ" => Self::Full,
            "K" => Self::KingSideOnly,
            "Q" => Self::QueenSideOnly,
            _ => Self::None,
        }
    }
}
