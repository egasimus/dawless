opt_mod::optional_module_flat!("cli": cli @ "s3kxl-cli.rs");
opt_mod::optional_module_flat!("tui": tui @ "s3kxl-tui.rs");

#[derive(PartialEq, Eq)]
pub enum DeviceModel { S900, S2000, S3000 }

pub struct Device<const M: DeviceModel>;

impl<const M: DeviceModel> Device<M> {
    /** Create and format an empty disk. */
    pub fn blank_disk (&self) -> Filesystem<M> {
        self.load_disk(&format::<M>())
    }
    /** Read the files from a disk image. */
    pub fn load_disk (&self, raw: &Vec<u8>) -> Filesystem<M> {
        Filesystem::new(raw.clone())
    }
}

pub fn akai_s900  () -> Device<{ DeviceModel::S900 }>  { Device }
pub fn akai_s2000 () -> Device<{ DeviceModel::S2000 }> { Device }
pub fn akai_s3000 () -> Device<{ DeviceModel::S3000 }> { Device }

pub fn disk_capacity (model: &DeviceModel) -> usize {
    match model {
        DeviceModel::S900 => 0x0C8000, // 819200 bytes
        _                 => 0x190000, // 1638400 bytes
    }
}

pub fn file_headers_offset (model: &DeviceModel) -> usize {
    match model {
        DeviceModel::S900 => 0x0000, // from byte 0
        _                 => 0x1400, // from byte 5120
    }
}

pub fn max_files (model: &DeviceModel) -> usize {
    match model {
        DeviceModel::S900 => 0x40,  // 64 entries
        _                 => 0x200, // 512 entries
    }
}

pub fn file_table_boundaries (model: &DeviceModel) -> (usize, usize) {
    match model {
        DeviceModel::S900 => (0x0600, 0x0c40), // from byte 1536 to 3136
        _                 => (0x0622, 0x0c5e), // from byte 1570 to 3166
    }
}

pub fn max_blocks (model: &DeviceModel) -> usize {
    match model {
        DeviceModel::S900 => 0x031c, // 796 blocks
        _                 => 0x062f, // 1583 blocks
    }
}

pub fn guess_model (volname: &[u8; 24]) -> DeviceModel {
    match volname[23] {
        0x00 => DeviceModel::S900,
        0x17 => DeviceModel::S2000,
        0x16 => DeviceModel::S3000,
        _    => panic!("could not determine device model from image data")
    }
}

/// Return a buffer containing a blank filesystem.
pub fn format <const M: DeviceModel> () -> Vec<u8> {

    // The empty buffer
    let mut raw   = vec![0x00; disk_capacity(&M)];

    // The current cursor position. Incremented by writing
    let mut index = 0x0000;

    // 0x000000 - 0x000600: 64 file headers for S900 compatibility
    for _ in 0..64 {
        index += put_vec(&mut raw, index, &[
            0x0A, 0x0A, 0x0A, 0x0A, 0x0A, 0x0A, 0x0A, 0x0A,
            0x0A, 0x0A, 0x0A, 0x0A, 0x00, 0x00, 0x06, 0x0A,
            0xFF, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, match M {
                DeviceModel::S900  => 0x00,
                _                  => 0x11 // first real data block,
            }
        ]);
    }

    // 0x000600 - 0x001280: 1600 block headers.
    assert_eq!(index, 0x0600);
    for _ in 0..1600 {
        index += put_vec(&mut raw, index, if index < 0x0622 {
            &[0x00, 0x40] // reserved for metadata
        } else {
            &[0x00, 0x00] // free
        });
    }

    // 0x001280 - 0x00128C: Volume name in AKAI format. (offset 4736)
    assert_eq!(index, 0x1280);
    index += put_vec(&mut raw, index, &[
        0x18, 0x19, 0x1E, 0x0A, 0x18, 0x0B, 0x17, 0x0F, 0x0E, 0x0A, 0x0A, 0x0A
    ]);

    // 0x00128C - 0x001400: Filesystem metadata (probably)
    assert_eq!(index, 0x128C);
    match M {
        DeviceModel::S900 => {
            // FIXME: this section is skipped on the S900 as per s2kdie sources.
            // so the assertions below will always fail
        },
        DeviceModel::S2000 => {
            let mut data = [0; 372];
            put(&mut data, 0x00, &[
                0x00, 0x00,
                0x00, 0x11,
                0x00, 0x01,
                0x01, 0x23,
                0x00, 0x00,
                0x32, 0x09,
                0x0C, 0xFF
            ]);
            index += put_vec(&mut raw, index, &data);
        },
        DeviceModel::S3000 => {
            let mut data = [0; 372];
            put(&mut data, 0x00, &[
                0x00, 0x00,
                0x32, 0x10,
                0x00, 0x01,
                0x01, 0x00,
                0x00, 0x00,
                0x32, 0x09,
                0x0C, 0xFF
            ]);
            index += put_vec(&mut raw, index, &data);
        }
    };

    // 0x001400 - 0x004400: 512 file headers
    assert_eq!(index, 0x1400);
    index += put_vec(&mut raw, index, &[0; 0x3000]);

    // 0x004400 - 0x190000: 1583 sectors for data, 1024 bytes each
    assert_eq!(index, 0x4400);
    put_vec(&mut raw, index, &[0; 0x18C20A]);

    raw
}

#[derive(Debug)]
pub struct Filesystem<const M: DeviceModel> {
    pub label: String,
    pub files: Vec<File>
}

impl<const M: DeviceModel> Filesystem<M> {

    /** Create a filesystem image. */
    pub fn new (raw: Vec<u8>) -> Self {
        Self {
            label: u8_to_string(&raw[0x1280..0x1280+12]),
            files: File::read_all::<M>(&raw.as_slice())
        }
    }

    #[cfg(feature = "cli")]
    pub fn list_files (self) -> Self {
        println!("\nLabel: {}", self.label);
        println!("Files:");
        for (i, file) in self.files.iter().enumerate() {
            println!("\n{: >4} {:<12} {:>8} bytes is a {:?}", i, file.name, file.data.len(), file.kind);
        }
        self
    }

    pub fn add_sample (self, name: &str, data: &[u8]) -> Self {
        self.add_file(name, FileType::S3000Sample, Sample::serialize(name, data))
    }

    pub fn add_file (mut self, name: &str, kind: FileType, data: Vec<u8>) -> Self {
        self.files.push(File { name: name.into(), kind, data });
        self
    }

    pub fn write_disk (self) -> Vec<u8> {
        // Get a blank disk image
        let mut data = format::<M>();
        // The block before the next block
        let mut block_id = 0x11;
        // Write each file
        for (file_index, file) in self.files.iter().enumerate() {
            // Split data into blocks
            let blocks = as_blocks(&file.data);
            // Store id of 1st block
            let start = block_id;
            // Write each block to the image
            for block in blocks.iter() {
                // Copy data to free block
                put_vec(&mut data, block_id * BLOCK_SIZE, block);
                // Update block table to point to block after that
                put_vec(&mut data, 0x0600 + block_id * 2, &(block_id + 1).to_le_bytes());
                // Get next free block
                block_id += 1;
            }
            // If we just wrote the last block of a file, mark the block as EOF in the table
            put_vec(&mut data, 0x0600 + (block_id - 1) * 2, &[0x00, 0xC0]);
            // Write the file header
            put_vec(&mut data, 0x1400 + file_index * 24, &FileHeader {
                name:  file.name.clone(),
                kind:  file.kind.clone(),
                size:  file.data.len() as u32,
                start: start as u16
            }.serialize());
        }
        data
    }

}

#[derive(Debug)]
pub struct File {
    pub name: String,
    pub kind: FileType,
    pub data: Vec<u8>
}

impl File {

    pub fn read_all <const M: DeviceModel> (raw: &[u8]) -> Vec<Self> {
        let mut files = vec![];
        let headers = FileHeader::read_all::<M>(&raw);
        let table = read_block_table::<M>(&raw);
        let blocks = as_blocks(&raw);
        for header in headers {
            files.push(File::read(header, &table, &blocks));
        }
        files
    }

    pub fn read (header: FileHeader, table: &Vec<BlockRecord>, blocks: &Vec<BlockData>) -> Self {
        // Buffer. Contents of blocks are copied into it.
        let mut data  = vec![0x00; header.size as usize];
        // Buffer write pointer
        let mut index = 0;
        // Block id
        let mut block = header.start;
        // Read bytes from linked blocks up to the file size
        while index < header.size as usize {
            // Copy block into buffer
            index += put_vec(&mut data, index, &blocks[block as usize]);
            // If there's a next block, repeat
            match table[block as usize] {
                BlockRecord::Next(next) => block = next,
                _ => break
            };
        }
        Self { name: header.name, kind: header.kind, data }
    }

}

#[derive(Debug)]
pub struct FileHeader {
    /// Name of file
    pub name:  String,
    /// Type of file
    pub kind:  FileType,
    /// Size of file in bytes
    pub size:  FileSize,
    /// Address of first block
    pub start: BlockIndex,
}

#[derive(Eq, PartialEq, Debug, Clone)]
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

pub type FileSize = u32;

pub type BlockIndex = u16;

#[derive(Debug, Eq, PartialEq)]
pub enum BlockRecord {
    Free,
    Reserved,
    Reserved2,
    EOF,
    Next(BlockIndex)
}

pub type BlockData = [u8; BLOCK_SIZE];

pub const BLOCK_SIZE: usize = 1024;

impl FileHeader {
    pub fn read_all <const M: DeviceModel> (raw: &[u8]) -> Vec<Self> {
        let offset = file_headers_offset(&M);
        let mut headers = vec![];
        // Read up to `max` FS records
        for entry in 0..max_files(&M) {
            match FileHeader::read(&raw[offset..], entry * 24) {
                Some(header) => headers.push(header),
                // Empty file header means we've reached the end
                None => break
            }
        }
        headers
    }
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
    pub fn write_all <const M: DeviceModel> (mut raw: Vec<u8>, headers: &Vec<Self>) -> Vec<u8> {
        let offset = file_headers_offset(&M);
        for entry in 0..max_files(&M) {
            match headers.get(entry) {
                None => put_vec(&mut raw, offset + entry * 24, &[0x00; 24]),
                Some(header) => put_vec(&mut raw, offset + entry * 24, &header.serialize())
            };
        }
        raw
    }
    pub fn serialize (&self) -> [u8; 24] {
        let mut data = [0x00; 24];
        let name = str_to_name(&self.name);
        // Fill first 16 bytes (filename) with spaces
        put(&mut data, 0x00, &[0x10; 16]);
        // Write filename (is it 12 or 16 chars after all?)
        put(&mut data, 0x00, &name[..usize::min(name.len(), 12)]);
        // Set file type
        data[0x10] = self.kind as u8;
        // Set file size (4 bytes)
        put(&mut data, 0x11, &self.size.to_le_bytes());
        // Set file start (2 bytes)
        put(&mut data, 0x14, &self.start.to_le_bytes());
        data
    }
}

pub fn read_block_table <const M: DeviceModel> (raw: &[u8]) -> Vec<BlockRecord> {
    let (start, end) = file_table_boundaries(&M);
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

pub fn write_block_table <const M: DeviceModel> (
    mut raw: Vec<u8>, table: &Vec<BlockRecord>
) -> Vec<u8> {
    let (start, end) = file_table_boundaries(&M);
    for index in 0..(end - start)/2 {
        match table[index] {
            BlockRecord::Free => {
                raw[start + index * 2]     = 0x00;
                raw[start + index * 2 + 1] = 0x00;
            },
            BlockRecord::Reserved => {
                raw[start + index * 2]     = 0x00;
                raw[start + index * 2 + 1] = 0x40;
            },
            BlockRecord::Reserved2 => {
                raw[start + index * 2]     = 0x00;
                raw[start + index * 2 + 1] = 0x80;
            },
            BlockRecord::EOF => {
                raw[start + index * 2]     = 0x00;
                raw[start + index * 2 + 1] = 0xc0;
            },
            BlockRecord::Next(block) => {
                let bytes = block.to_le_bytes();
                raw[start + index * 2]     = bytes[0];
                raw[start + index * 2 + 1] = bytes[1];
            }
        }
    }
    raw
}

/// Split a slice into blocks of 1024 bytes
pub fn as_blocks (data: &[u8]) -> Vec<BlockData> {
    let mut blocks = vec![];
    let mut pointer = 0;
    while pointer < data.len() {
        let mut block = [0; 1024];
        put(&mut block, 0, &data[pointer..]);
        blocks.push(block);
        pointer += 1024;
    }
    blocks
}

pub fn write_blocks <const M: DeviceModel> (mut data: Vec<u8>, blocks: &Vec<BlockData>) -> Vec<u8> {
    for i in 0x11..max_blocks(&M) {
        put(&mut data, i * 1024, &blocks[i]);
    }
    data
}
#[derive(Debug)]
pub struct Multi {}
#[derive(Debug)]
pub struct Program {
    id:                   u8,       // 00      C       1       program header id
    addr_kg1:             u16,      // 01-02   x2      n/a     1st keygroup address (internal)
    name:                 String,   // 03-0e   a12             program name
    midi_program:         u8,       // 0f      C       0       MIDI program number (0-127)
    midi_channel:         u8,       // 10      C       0       MIDI channel (0-15, ff=omni)
    polyphony:            u8,       // 11      C       31      polyphony (1-32; 1-16 in S1000)
    priority:             u8,       // 12      C       1       priority (0=low, 1=normal, 2=high, 3=hold)
    range_low:            u8,       // 13      C       24      play range low (24-127 = C0-G8)
    range_hight:          u8,       // 14      C       127     play range high (24-127 = C0-G8)
    octave:               u8,       // 15      C       0       play octave (keyboard) shift (+/-2)
    output:               u8,       // 16      C       255     indivisual output (0-7, ff=off)
    volume:               u8,       // 17      C       99      stereo level
    pan:                  u8,       // 18      C       0       stereo pan
    loudness:             u8,       // 19      C       80      loudness
    vel_to_loudness:      u8,       // 1a      C       20      velocity > loud
    key_to_loudness:      u8,       // 1b      C       0       key > loud
    pressure_to_loudness: u8,       // 1c      C       0       pressure > loud
    pan_lfo_rate:         u8,       // 1d      C       0       pan LFO rate
    pan_depth:            u8,       // 1e      C       99      pan depth
    pan_lfo_delay:        u8,       // 1f      C       0       pan LFO delay
    key_to_pan:           u8,       // 20      C       0       key > pan position 
    lfo_speed:            u8,       // 21      C       50      LFO speed
    lfo_depth:            u8,       // 22      C       0       LFO fixed depth
    lfo_delay:            u8,       // 23      C       0       LFO delay
    mod_to_depth:         u8,       // 24      C       30      modwheel > depth
    pressure_to_depth:    u8,       // 25      C       0       pressure > depth
    velocity_to_depth:    u8,       // 26      C       0       velocity > depth
    bend_to_pitch:        u8,       // 27      C       2       bendwheel > pitch
    pressure_to_pitch:    u8,       // 28      C       0       pressure > pitch
    keygroup_crossfade:   u8,       // 29      C       0       keygroup crossfade (0=off, 1=on)
    number_of_keygroups:  u8,       // 2a      C               # of keygroups (1-99)
    temp_program_number:  u8,       // 2b      C       n/a     temporary program number (internal)
    temperament:          [u8; 12], // 2c-37   C12             key temperament
    echo:                 u8,       // 38      C       0       echo output level (0=off, 1=on)
    modwheel_pan_amount:  u8,       // 39      C       0       modwheel pan amount
    retrigger:            u8,       // 3a      C       0       sample start coherence (0=off, 1=on)
    lfo_desync:           u8,       // 3b      C       0       LFO de-sync (0=off, 1=on) (def. 0)
    pitch_law:            u8,       // 3c      C       0       pitch law
    voice_assign_algo:    u8,       // 3d      C       0       voice assign algorithm (0=oldest, 1=quietest)
    pedal_to_loudness:    u8,       // 3e      C       10      soft pedal loudness reduction 
    pedal_to_attack:      u8,       // 3f      C       10      soft pedal attack stretch
    pedal_to_filter:      u8,       // 40      C       10      soft pedal filter close
    tune_offset:          u16,      // 41-42   v       0       tune offset
    key_to_lfo_rate:      u8,       // 43      C       0       key > LFO rate
    key_to_lfo_depth:     u8,       // 44      C       0       key > LFO depth
    key_to_lfo_delay:     u8,       // 45      C       0       key > LFO delay
    voice_output_scale:   u8,       // 46      C       50      voice output scale 
    stereo_output_scale:  u8,       // 47      C       0       stereo output scale
    keygroup: Vec<Keygroup>
}

#[derive(Debug)]
pub struct Keygroup {
    // 0000-0021       keygroup common data
    // 0022-
    // 
    // 00      C       2       keygroup block id
    // 01-02   v       n/a     next keygroup block address (internal)
    // 03      C       24      keyrange low
    // 04      C       127     keyrange high
    // 05-06   v       0       tune offset
    // 07      C       99      filter freq.
    // 08      C       0       key > filter freq.
    // 09      C       0       velocity > filter freq.
    // 0a      C       0       pressure > filter freq.
    // 0b      C       0       envelope > filter freq.
    // 0c      C       25      amp. attack
    // 0d      C       50      amp. decay
    // 0e      C       99      amp. sustain
    // 0f      C       45      amp. release
    // 10      C       0       velocity > amp. attack
    // 11      C       0       velocity > amp. release
    // 12      C       0       off velocity > amp. release
    // 13      C       0       key > decay & release 
    // 14      C       0       filter attack
    // 15      C       50      filter decay 
    // 16      C       99      filter sustain
    // 17      C       45      filter release
    // 18      C       0       velocity > filter attack
    // 19      C       0       velocity > filter relase
    // 1a      C       0       off velocity > fiter release
    // 1b      C       0       key > decay & release
    // 1c      C       25      velocity > filter envelope output
    // 1d      C       0       envelope > pitch 
    // 1e      C       1       velocity zone crossfade (0=off, 1=on)
    // 1f      C       n/a     # of velocity zones (internal)
    // 20      C       n/a     internal
    // 21      C       n/a     internal
    // 
    // 
    // 83      C       0       fixed rate detune
    // 84      C       0       attack hold until loop
    // 85-88   C4      0       constant pitch for zone 1--4 (0=track, 1=const); 84??
    // 89-8c   C4      0       output number offset for zone 1--4
    // 8d-94   v4      0       velocity > sample start
    // 95      C       0       velocity > loudness offset
    // 96-bf                   ??
    // 97      C       0       vel.  > filter freq.
    // 98      C       0       pres. > filter freq.
    // 99      C       0       env.  > filter freq.
    zones: [Zone; 4]
}

#[derive(Debug)]
pub struct Zone {
    // 22-2d   A12             sample name
    // 2e      C       0       velocity range low
    // 2f      C       127     velocity range high
    // 30-31   v       0       tune offset
    // 32      C       0       loudness offset
    // 33      C       0       filter freq. offset
    // 34      c       0       pan offset
    // 35      C       0       loop in relase
    // 36      C       n/a     low velocity xfade factor (intarnal)
    // 37      C       n/a     low velocity xfade factor (intarnal)
    // 38-39   v       n/a     sample header block address (intarnal)
    // 
    // 3a-52                   velocity zone 2
    // 53-69                   velocity zone 3
    // 6a-82                   velocity zone 4
}

pub fn sample_header_length (model: &DeviceModel) -> usize {
    match model {
        DeviceModel::S900 => 0x3c, // 796 blocks
        _                 => 0xbe, // 1583 blocks
    }
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

impl<'a> Sample<'a> {
    pub fn serialize (name: &str, data: &[u8]) -> Vec<u8> {
        let header_length = 0xbe;
        let compression = data[20];
        if compression != 1 {
            panic!("uncompressed wavs only")
        }
        let bitrate = data[34];
        if bitrate != 16 {
            panic!("16-bit wavs only")
        }
        let sample_rate = u32::from_le_bytes([data[24], data[25], data[26], data[27]]);
        if sample_rate != 44100 {
            panic!("44.1kHz wavs only")
        }
        let channels = data[22];
        if channels != 1 {
            panic!("mono wavs only")
        }
        let contents = &data[44..];
        let length = ((contents.len()/2) as u32).to_le_bytes();
        let max = header_length + contents.len();
        let mut output = vec![0x00; max];
        output[0x00] = 0x03; // format (S3000)
        let channels = data[22];
        if channels != 1 {
            panic!("mono wavs only")
        }
        output[0x01] = channels; // channels
        output[0x02] = 0x3C;     // original pitch
        // name
        let name_akai = str_to_name(name);
        put_vec(&mut output, 0x03, &name_akai[..usize::min(name_akai.len(), 12)]);
        output[0x0f] = 0x80; // valid
        output[0x10] = 0x01; // loops
        output[0x11] = 0x00; // first loop
        output[0x12] = 0x00; // dummy
        output[0x13] = 0x02; // don't loop
        output[0x14] = 0x00; // tune cent
        output[0x15] = 0x00; // tune semi
        // data abs. start addr. (internal?)
        put_vec(&mut output, 0x16, &[0x00, 0x04, 0x01, 0x00]);
        // set sample length
        put_vec(&mut output, 0x1a, &length);
        // set sample start
        put_vec(&mut output, 0x1e, &[0x00, 0x00, 0x00, 0x00]);
        // set sample end
        put_vec(&mut output, 0x22, &length);
        // set sample rate
        put_vec(&mut output, 0x8a, &sample_rate.to_le_bytes());
        // copy sample data
        put_vec(&mut output, header_length, &contents);

        output
    }
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
