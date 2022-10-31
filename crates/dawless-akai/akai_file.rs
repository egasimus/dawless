use crate::*;

#[derive(Debug)]
pub struct FileRecord {
    /// Name of file
    pub name:  String,
    /// Type of file
    pub kind:  FileType,
    /// Size of file in bytes
    pub size:  u32,
    /// Address of first block
    pub start: u16,
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

#[derive(Debug, Eq, PartialEq)]
pub enum BlockRecord {
    Free,
    Reserved,
    Reserved2,
    EOF,
    Next(u16)
}

pub type BlockData = [u8; BLOCK_SIZE];

pub const BLOCK_SIZE: usize = 1024;

pub fn load_file_headers (raw: &[u8], max: usize) -> Vec<FileRecord> {
    let mut headers = vec![];
    // Read up to `max` FS records
    for entry in 0..max {
        match FileRecord::read(raw, entry * 24) {
            Some(header) => headers.push(header),
            // Empty file header means we've reached the end
            None => break
        }
    }
    headers
}

pub fn save_file_headers (model: &DeviceModel, raw: Vec<u8>) -> Vec<u8> {
    raw
}

impl FileRecord {
    fn read (raw: &[u8], offset: usize) -> Option<Self> {
        if raw[offset] == 0x00 {
            return None
        } else {
            let head = &raw[offset..offset+24];
            Some(Self {
                name:  u8_to_string(&head[..12]),
                kind:  file_type(head[0x10]),
                size:  u32::from_le_bytes([head[0x11], head[0x12], head[0x13], 0x00]),
                start: u16::from_le_bytes([head[0x14], head[0x15]]),
            })
        }
    }
}

pub fn load_block_table (model: &DeviceModel, raw: &[u8]) -> Vec<BlockRecord> {
    let (start, end) = file_table_boundaries(model);
    println!("{start} {end}");
    let table = &raw[start..end];
    let mut blocks = Vec::with_capacity(table.len() / 2);
    for address in (0..table.len()).step_by(2) {
        match (table[address], table[address+1]) {
            // reserved for system
            (0x00, 0x00) => {
                blocks.push(BlockRecord::Free);
            },
            // reserved for system
            (0x00, 0x40) => {
                blocks.push(BlockRecord::Reserved);
            },
            // reserved for 2nd file entry
            (0x00, 0x80) => {
                blocks.push(BlockRecord::Reserved2);
            },
            // end of file
            (0x00, 0xc0) => {
                blocks.push(BlockRecord::EOF);
            },
            // file continues at
            _ => {
                blocks.push(BlockRecord::Next(
                    u16::from_le_bytes([table[address], table[address+1]])
                ))
            }
        }
    }
    blocks[0] = BlockRecord::Reserved;
    blocks[1] = BlockRecord::Reserved;
    blocks[2] = BlockRecord::Reserved;
    blocks
}

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
