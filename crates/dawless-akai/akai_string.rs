/// The characters allowed in filenames
pub const AKAI_CHARSET: [char; 41] = [
    '0','1','2','3','4','5','6','7','8','9',
    ' ',
    'A','B','C','D','E','F','G','H','I','J',
    'K','L','M','N','O','P','Q','R','S','T',
    'U','V','W','X','Y','Z','#','+','-','.',
];

/// Convert an AKAI string to an ASCII string
#[inline]
pub fn u8_to_string (chars: &[u8]) -> String {
    chars.iter().map(|x| AKAI_CHARSET[*x as usize]).collect()
}

/// Convert an ASCII string to an Akai string
#[inline]
pub fn str_to_name (chars: &str) -> Vec<u8> {
    let to_akai_char = |x: char| match AKAI_CHARSET.iter().position(|&y|y==x.to_ascii_uppercase()) {
        Some(z) => z, None => 10,
    } as u8;
    chars.chars().map(to_akai_char).collect()
}

/// Fill a buffer with content starting from offset.
#[inline]
pub fn put (buffer: &mut [u8], offset: usize, content: &[u8]) -> usize {
    let mut count = 0;
    for (index, value) in content.iter().enumerate() {
        if offset + index >= buffer.len() {
            break
        }
        count += 1;
        buffer[offset + index] = *value;
    }
    count
}

/// Fill a vector with content starting from offset.
#[inline]
pub fn put_vec (buffer: &mut Vec<u8>, offset: usize, content: &[u8]) -> usize {
    let mut count = 0;
    for (index, value) in content.iter().enumerate() {
        if offset + index >= buffer.len() {
            break
        }
        count += 1;
        buffer[offset + index] = *value;
    }
    count
}

/// Fill a vector with content starting from offset.
#[inline]
pub fn put_vec_max (max: usize, buffer: &mut Vec<u8>, offset: usize, content: &[u8]) -> usize {
    let mut count = 0;
    for (index, value) in content.iter().enumerate() {
        if offset + index >= max {
            break
        }
        count += 1;
        buffer[offset + index] = *value;
    }
    count
}
