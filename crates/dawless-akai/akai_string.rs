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
