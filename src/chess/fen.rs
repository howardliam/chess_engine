use regex::Regex;

pub fn fen_is_valid(fen: String) -> bool {
    let regex = Regex::new(
        r"^([rnbqkpRNBQKP1-8]{1,8}\/?){8}\s+(b|w)\s+(-|K?Q?k?q)\s+(-|[a-h][3-6])\s+(\d+)\s+(\d+)$",
    )
    .unwrap();

    regex.is_match(fen.as_str())
}
