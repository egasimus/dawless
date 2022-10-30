pub const CHARSET: [char; 256] = [
    '▁', // 0x00	
    '☺', // 0x01	
    '☻', // 0x02	
    '♥', // 0x03	
    '♦', // 0x04	
    '♣', // 0x05	
    '♠', // 0x06	
    '•', // 0x07
    '◘', // 0x08	
    '○', // 0x09	
    '◙', // 0x0A	
    '♂', // 0x0B	
    '♀', // 0x0C	
    '♪', // 0x0D	
    '♫', // 0x0E	
    '☼', // 0x0F
    '►', // 0x10	
    '◄', // 0x11	
    '↕', // 0x12	
    '‼', // 0x13	
    '¶', // 0x14	
    '§', // 0x15	
    '▬', // 0x16	
    '↨', // 0x17
    '↑', // 0x18	
    '↓', // 0x19	
    '→', // 0x1A	
    '←', // 0x1B	
    '∟', // 0x1C	
    '↔', // 0x1D	
    '▲', // 0x1E	
    '▼', // 0x1F
    ' ', // 0x20	
    '!', // 0x21	
    '"', // 0x22	
    '#', // 0x23	
    '$', // 0x24	
    '%', // 0x25	
    '&', // 0x26	
    '\'', // 0x27
    '(', // 0x28	
    ')', // 0x29	
    '*', // 0x2A	
    '+', // 0x2B	
    ',', // 0x2C	
    '-', // 0x2D	
    '.', // 0x2E	
    '/', // 0x2F
    '0', // 0x30	
    '1', // 0x31	
    '2', // 0x32	
    '3', // 0x33	
    '4', // 0x34	
    '5', // 0x35	
    '6', // 0x36	
    '7', // 0x37
    '8', // 0x38	
    '9', // 0x39	
    ':', // 0x3A	
    ';', // 0x3B	
    '<', // 0x3C	
    '=', // 0x3D	
    '>', // 0x3E	
    '?', // 0x3F
    '@', // 0x40	
    'A', // 0x41	
    'B', // 0x42	
    'C', // 0x43	
    'D', // 0x44	
    'E', // 0x45	
    'F', // 0x46	
    'G', // 0x47
    'H', // 0x48	
    'I', // 0x49	
    'J', // 0x4A	
    'K', // 0x4B	
    'L', // 0x4C	
    'M', // 0x4D	
    'N', // 0x4E	
    'O', // 0x4F
    'P', // 0x50	
    'Q', // 0x51	
    'R', // 0x52	
    'S', // 0x53	
    'T', // 0x54	
    'U', // 0x55	
    'V', // 0x56	
    'W', // 0x57
    'X', // 0x58	
    'Y', // 0x59	
    'Z', // 0x5A	
    '[', // 0x5B	
    '\\', // 0x5C	
    ']', // 0x5D	
    '^', // 0x5E	
    '_', // 0x5F
    '`', // 0x60	
    'a', // 0x61	
    'b', // 0x62	
    'c', // 0x63	
    'd', // 0x64	
    'e', // 0x65	
    'f', // 0x66	
    'g', // 0x67
    'h', // 0x68	
    'i', // 0x69	
    'j', // 0x6A	
    'k', // 0x6B	
    'l', // 0x6C	
    'm', // 0x6D	
    'n', // 0x6E	
    'o', // 0x6F
    'p', // 0x70	
    'q', // 0x71	
    'r', // 0x72	
    's', // 0x73	
    't', // 0x74	
    'u', // 0x75	
    'v', // 0x76	
    'w', // 0x77
    'x', // 0x78	
    'y', // 0x79	
    'z', // 0x7A	
    '{', // 0x7B	
    '|', // 0x7C	
    '}', // 0x7D	
    '~', // 0x7E	
    '⌂', // 0x7F
    '█', // 0x80	
    '⡀', // 0x81	
    '⢀', // 0x82	
    '⣀', // 0x83	
    '⠠', // 0x84	
    '⡠', // 0x85	
    '⢠', // 0x86	
    '⣠', // 0x87
    '⠄', // 0x88	
    '⡄', // 0x89	
    '⢄', // 0x8A	
    '⣄', // 0x8B	
    '⠤', // 0x8C	
    '⡤', // 0x8D	
    '⢤', // 0x8E	
    '⣤', // 0x8F
    '⠁', // 0x90	
    '⡁', // 0x91	
    '⢁', // 0x92	
    '⣁', // 0x93	
    '⠡', // 0x94	
    '⡡', // 0x95	
    '⢡', // 0x96	
    '⣡', // 0x97
    '⠅', // 0x98	
    '⡅', // 0x99	
    '⢅', // 0x9A	
    '⣅', // 0x9B	
    '⠥', // 0x9C	
    '⡥', // 0x9D	
    '⢥', // 0x9E	
    '⣥', // 0x9F
    '⠃', // 0xA0	
    '⡃', // 0xA1	
    '⢃', // 0xA2	
    '⣃', // 0xA3	
    '⠣', // 0xA4	
    '⡣', // 0xA5	
    '⢣', // 0xA6	
    '⣣', // 0xA7
    '⠇', // 0xA8	
    '⡇', // 0xA9	
    '⢇', // 0xAA	
    '⣇', // 0xAB	
    '⠧', // 0xAC	
    '⡧', // 0xAD	
    '⢧', // 0xAE	
    '⣧', // 0xAF
    '⠉', // 0xB0	
    '⡉', // 0xB1	
    '⢉', // 0xB2	
    '⣉', // 0xB3	
    '⠩', // 0xB4	
    '⡩', // 0xB5	
    '⢩', // 0xB6	
    '⣩', // 0xB7
    '⠍', // 0xB8	
    '⡍', // 0xB9	
    '⢍', // 0xBA	
    '⣍', // 0xBB	
    '⠭', // 0xBC	
    '⡭', // 0xBD	
    '⢭', // 0xBE	
    '⣭', // 0xBF
    '⠊', // 0xC0	
    '⡊', // 0xC1	
    '⢊', // 0xC2	
    '⣊', // 0xC3	
    '⠪', // 0xC4	
    '⡪', // 0xC5	
    '⢪', // 0xC6	
    '⣪', // 0xC7
    '⠎', // 0xC8	
    '⡎', // 0xC9	
    '⢎', // 0xCA	
    '⣎', // 0xCB	
    '⠮', // 0xCC	
    '⡮', // 0xCD	
    '⢮', // 0xCE	
    '⣮', // 0xCF
    '⠑', // 0xD0	
    '⡑', // 0xD1	
    '⢑', // 0xD2	
    '⣑', // 0xD3	
    '⠱', // 0xD4	
    '⡱', // 0xD5	
    '⢱', // 0xD6	
    '⣱', // 0xD7
    '⠕', // 0xD8	
    '⡕', // 0xD9	
    '⢕', // 0xDA	
    '⣕', // 0xDB	
    '⠵', // 0xDC	
    '⡵', // 0xDD	
    '⢵', // 0xDE	
    '⣵', // 0xDF
    '⠚', // 0xE0	
    '⡚', // 0xE1	
    '⢚', // 0xE2	
    '⣚', // 0xE3	
    '⠺', // 0xE4	
    '⡺', // 0xE5	
    '⢺', // 0xE6	
    '⣺', // 0xE7
    '⠞', // 0xE8	
    '⡞', // 0xE9	
    '⢞', // 0xEA	
    '⣞', // 0xEB	
    '⠾', // 0xEC	
    '⡾', // 0xED	
    '⢾', // 0xEE	
    '⣾', // 0xEF
    '⠛', // 0xF0	
    '⡛', // 0xF1	
    '⢛', // 0xF2	
    '⣛', // 0xF3	
    '⠻', // 0xF4	
    '⡻', // 0xF5	
    '⢻', // 0xF6	
    '⣻', // 0xF7
    '⠟', // 0xF8	
    '⡟', // 0xF9	
    '⢟', // 0xFA	
    '⣟', // 0xFB	
    '⠿', // 0xFC	
    '⡿', // 0xFD	
    '⢿', // 0xFE	
    '⣿', // 0xFF
];

pub trait BrailleDump<'a>: std::iter::IntoIterator<Item = &'a u8> + Sized {
    fn into_braille_dump (self) -> String {
        self.into_iter().map(|x| CHARSET[*x as usize]).collect()
    }
}

impl<'a> BrailleDump<'a> for &'a [u8] {}

impl<'a, const N: usize> BrailleDump<'a> for &'a [u8; N] {}
