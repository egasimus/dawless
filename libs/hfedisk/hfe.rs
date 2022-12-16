use binread::BinRead;
use brailledump::CHARSET;

#[derive(Debug, BinRead)]
#[br(magic = b"HXCPICFE")]
pub struct Header {
    /// Revision 0
    pub format_revision:       u8,
    /// Number of tracks in file
    pub tracks:                u8,
    /// Number of valid sides (not used by emulator)
    pub sides:                 u8,
    /// Track encoding mode (used for write support)
    pub encoding:              TrackEncoding,
    /// Bitrate in Kbit/s., e.g. 250 = 250000b/s; max: 500
    pub bitrate:               u16,
    /// Rotation per minute (not used by emulator)
    pub rpm:                   u16,
    /// Floppy interface mode
    pub mode:                  InterfaceMode,
    /// Free
    pub dnu:                   u8,
    /// Offset of track list LUT in blocks of 512 bytes (e.g. 1 = 0x200)
    pub track_list_offset:     u8,
    /// Is write protected?
    pub writable:              u8,
    /// 0xFF - single step, 0x00 - double step
    pub step:                  StepMode,
    /// 0x00 - use alt. encoding for side 0
    pub track0_s0_altencoding: UseAltEncoding,
    /// Alternate encoding for side 0
    pub track0_s0_encoding:    TrackEncoding,
    /// 0x00 - use alt. encoding for side 1
    pub track0_s1_altencoding: UseAltEncoding,
    /// Alternate encoding for side 1
    pub track0_s1_encoding:    TrackEncoding,
}

#[derive(Debug, BinRead)]
#[br(repr = u8)]
pub enum UseAltEncoding {
    Yes = 0x00,
    No  = 0xFF,
}

#[derive(Debug, BinRead)]
#[br(repr = u8)]
pub enum StepMode {
    Double = 0x00,
    Single = 0xFF,
}

#[derive(Debug, BinRead)]
#[br(repr = u8)]
pub enum InterfaceMode {
    IBMPCDD           = 0x00,
    IBMPCHD           = 0x01,
    AtariSTDD         = 0x02,
    AtariSTHD         = 0x03,
    AmigaDD           = 0x04,
    AmigaHD           = 0x05,
    CPCDD             = 0x06,
    GenericShuggartDD = 0x07,
    IBMPCED           = 0x08,
    MSX2DD            = 0x09,
    C64DD             = 0x0A,
    EmuShugart        = 0x0B,
    S950DD            = 0x0C,
    S950HD            = 0x0D,
    Disable           = 0xFE,
}

#[derive(Debug, BinRead)]
#[br(repr = u8)]
pub enum TrackEncoding {
    ISOIBMMFM  = 0x00,
    AmigaMFM   = 0x01,
    ISOIBMFM   = 0x02,
    EmuFM      = 0x03,
    Unknown    = 0xFF,
}

/// Track offset LUT entry
#[derive(Debug, BinRead, Clone)]
pub struct TrackInfo {
    /// Offset of the track data in block of 512 bytes (e.g. 2=0x400)
    pub offset: u16,
    /// Length of the track data in bytes
    pub length: u16,
}

#[allow(dead_code)]
#[derive(Debug, BinRead)]
pub struct TrackBlock {
    #[br(count = 256)]
    side_0: Vec<u16>,
    #[br(count = 256)]
    side_1: Vec<u16>
}

impl std::fmt::Display for TrackBlock {
    fn fmt (&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut bits_0 = String::new();
        for byte in self.side_0.iter() {
            let byte = CHARSET[mfm_decode_byte(byte) as usize];
            bits_0.push_str(&format!("{byte}"));
        }
        let mut bits_1 = String::new();
        for byte in self.side_1.iter() {
            let byte = CHARSET[mfm_decode_byte(byte) as usize];
            bits_1.push_str(&format!("{byte}"));
        }
        write!(f, "\n{}\n{}", bits_0, bits_1)
    } 
}

pub fn mfm_decode_byte (i: &u16) -> u8 {
    (0b00000000
        | (0b00000001 & i)
        | ((0b00000100 & i) >> 1)
        | ((0b00010000 & i) >> 2)
        | ((0b01000000 & i) >> 3)
        | (((0b00000001 <<  8) & i) >> 4)
        | (((0b00000001 << 10) & i) >> 5)
        | (((0b00000001 <<  6) & i) >> 4)
        | (((0b00000001 << 14) & i) >> 7)) as u8
}

#[derive(Debug)]
pub struct Track {
    pub info:   TrackInfo,
    pub blocks: Vec<TrackBlock>
}
