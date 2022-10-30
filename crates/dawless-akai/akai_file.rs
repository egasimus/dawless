pub fn file_type (byte: u8) -> FileType {
    match byte {
        0x00 => FileType::Deleted,
        0x63 => FileType::OS,
        0x64 => FileType::Drum,
        0x70 => FileType::S1000Program,
        0x71 => FileType::QL,
        0x73 => FileType::S1000Sample,
        0x74 => FileType::TL,
        0x78 => FileType::EffectsFile,
        0xED => FileType::MultiFile,
        0xF0 => FileType::S3000Program,
        0xF3 => FileType::S3000Sample,
        _ => panic!("unknown file type: 0x{byte:02X?}")
    }
}

#[derive(Eq, PartialEq, Debug)]
pub enum FileType {
    Deleted      = 0x00,
    OS           = 0x63,
    Drum         = 0x64,
    S1000Program = 0x70,
    QL           = 0x71,
    S1000Sample  = 0x73,
    TL           = 0x74,
    EffectsFile  = 0x78,
    MultiFile    = 0xED,
    S3000Program = 0xF0,
    S3000Sample  = 0xF3,
}

#[derive(Debug)]
pub struct Sample<'a> {
    pub name:        String,
    pub size:        u32,
    pub data:        &'a [u8],
    pub sample_rate: SampleRate,
    pub loop_mode:   LoopMode,
    pub tuning_semi: u8,
    pub tuning_cent: u8,
    pub length:      u32
}

#[derive(Debug)]
pub enum SampleRate {
    Hz22050,
    Hz44100
}

#[derive(Debug)]
pub enum LoopMode {
    Normal,
    UntilRelease,
    NoLoop,
    PlayToEnd
}
