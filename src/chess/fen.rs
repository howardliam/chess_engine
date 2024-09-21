use regex::Regex;

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
    pub halfmove_clock: String,
    pub fullmove_number: String,
}

impl Fen {
    // TODO add errors
    pub fn from(fen: String) -> Result<Self, ()> {
        if !fen_is_valid(&fen) {
            return Err(());
        }

        let fragments = fen.split(' ').collect::<Vec<&str>>();

        if fragments.len() != 6 {
            return Err(());
        }

        Ok(Self {
            board_layout: fragments[0].to_owned(),
            side_to_move: fragments[1].to_owned(),
            castling_rights: fragments[2].to_owned(),
            en_passant_square: fragments[3].to_owned(),
            halfmove_clock: fragments[4].to_owned(),
            fullmove_number: fragments[5].to_owned(),
        })
    }
}
