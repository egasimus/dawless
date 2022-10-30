pub const AKAI_CHARSET: [char; 41] = [
    '0','1','2','3','4','5','6','7','8','9',
    ' ',
    'A','B','C','D','E','F','G','H','I','J',
    'K','L','M','N','O','P','Q','R','S','T',
    'U','V','W','X','Y','Z','#','+','-','.',
];

pub fn u8_to_string (chars: &[u8]) -> String {
    chars.iter().map(|x| AKAI_CHARSET[*x as usize]).collect()
}

pub fn str_to_name (chars: &str) -> Vec<u8> {
    let to_akai_char = |x: char| match AKAI_CHARSET.iter().position(|&y|y==x.to_ascii_uppercase()) {
        Some(z) => z, None => 10,
    } as u8;
    chars.chars().map(to_akai_char).collect()
}
