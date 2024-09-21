use core::fmt;

use regex::Regex;

use crate::errors::InvalidFenError;

pub fn fen_is_valid(fen: &String) -> bool {
    let regex = Regex::new(
        r"^([rnbqkpRNBQKP1-8]{1,8}\/?){8}\s+(b|w)\s+(-|K?Q?k?q)\s+(-|[a-h][3-6])\s+(\d+)\s+(\d+)$",
    )
    .unwrap();

    regex.is_match(fen.as_str())
}

pub struct Fen {
    pub board_layout: String,
    pub side_to_move: String,
    pub castling_rights: String,
    pub en_passant_square: String,
    pub halfmove_clock: i32,
    pub fullmove_number: i32,
}

impl Fen {
    pub const STARTING_POSITION: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
}

impl TryFrom<String> for Fen {
    type Error = InvalidFenError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if !fen_is_valid(&value) {
            return Err(InvalidFenError);
        }

        let fragments = value.split(' ').collect::<Vec<&str>>();

        if fragments.len() != 6 {
            return Err(InvalidFenError);
        }

        let halfmove_clock = fragments[4]
            .parse::<i32>()
            .expect("Halfmove clock should have parsed just fine");
        let fullmove_number = fragments[5]
            .parse::<i32>()
            .expect("Fullmove number should have parsed just fine");

        Ok(Self {
            board_layout: fragments[0].to_owned(),
            side_to_move: fragments[1].to_owned(),
            castling_rights: fragments[2].to_owned(),
            en_passant_square: fragments[3].to_owned(),
            halfmove_clock,
            fullmove_number,
        })
    }
}
