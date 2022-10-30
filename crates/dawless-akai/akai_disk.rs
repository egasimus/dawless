use crate::*;

pub trait DeviceDisk<const M: DeviceModel> {

    /** Create and format an empty disk. */
    fn blank_disk (&self) -> DiskImage<M> {
        let mut disk = DiskImage::new(
            Vec::with_capacity(Self::disk_size()),
            Self::label_start(),
            Self::header_start(),
            Self::max_entries(),
            Self::fat_start(),
            Self::fat_end()
        );
        disk.raw.extend_from_slice(&self.section1());
        disk.raw.extend_from_slice(&self.section2());
        disk.raw.extend_from_slice(&self.section3());
        disk.raw.extend_from_slice(&self.section4());
        disk.raw.extend_from_slice(&self.section5());
        disk.raw.extend_from_slice(&self.section6());
        disk
    }

    /** Read the files from a disk image. */
    fn load_disk (&self, raw: Vec<u8>) -> DiskImage<M> {
        DiskImage::new(
            raw,
            Self::label_start(),
            Self::header_start(),
            Self::max_entries(),
            Self::fat_start(),
            Self::fat_end()
        )
    }

    fn disk_size () -> usize;

    /** Disk header. */
    fn section1 (&self) -> [u8; 24];

    /** Bunch of empty space. (what does it mean?) */
    fn section2 (&self) -> [u8; 3166] {
        [0; 3166]
    }

    /** Volume name in AKAI format. (offset 4736) */
    fn section3 (&self) -> [u8; 12] {
        [0x18, 0x19, 0x1E, 0x0A, 0x18, 0x0B, 0x17, 0x0F, 0x0E, 0x0A, 0x0A, 0x0A]
    }

    /** Not supported on S900, different on S2000 vs S3000. (what does it mean?) */
    fn section4 (&self) -> [u8; 372];

    /** 512 possible directory entries. (offset 5120) */
    fn section5 (&self) -> [u8; 12288] {
        [0; 12288]
    }

    /** 1583 * 1024 byte sectors for data. (offset 17408?) */
    fn section6 (&self) -> [u8; 1620992] {
        [0; 1620992]
    }

    fn max_entries () -> usize;

    fn header_start () -> usize;

    fn max_blocks () -> usize;

    fn fat_start () -> usize;

    fn fat_end () -> usize;

    fn label_start () -> usize {
        4736
    }

    // /// Reads the metadata and 1st block of each file in the disk image.
    // /// Corresponds to 1st loop of s2kdie importimage().
    //fn load_disk_head (disk: &mut DiskImage<M>) {
        //let headers = load_headers(
            //&disk.raw.as_slice()[Self::fat_offset()..],
            //Self::fat_max_entries()
        //);
        //let raw = disk.raw.as_slice();
        //let max_entries = Self::fat_max_entries();
        //let data_offset = Self::fat_offset();
        //let max_blocks  = Self::fat_max_blocks();
        //// Used to determine number of remaining blocks
        //let mut last_block = 0;
        //// Read up to `max_entries` FS records
        //let mut i = 0;
        //while i < max_entries {
            //let entry_offset = data_offset + i * 24;
            //// If we've reached an empty entry we're past the end
            //if raw[entry_offset] == 0x00 {
                //break
            //}
            //let mut head: [u8; 24] = [0; 24];
            //head.copy_from_slice(&raw[entry_offset..entry_offset+24]);
            //disk.head.push(head);
            //let size = u32::from_le_bytes([head[17], head[18], head[19], 0x00]);
            //disk.size.push(size);
            //let block_index = u32::from_le_bytes([head[20], head[21], 0x00, 0x00]);
            //let block_start = (block_index * 1024) as usize;
            //let block_end   = ((block_index + 1) * 1024) as usize;
            //let block_data  = &raw[block_start..block_end];
            //disk.data.push(block_data.to_vec());
            //last_block += size / 1024;
            //i += 1;
        //}
        //disk.free = max_blocks - last_block as usize;
    //}

    // /// Reads subsequent blocks (fragments) from the image.
    // /// Corresponds to 2nd loop of s2kdie importimage()
    //fn load_disk_body (disk: &mut DiskImage<M>) {
        //let startb = Self::fat_start();
        //let endb   = Self::fat_end();
        //let raw    = disk.raw.as_slice();
        //let tmap   = &raw[startb..endb-startb];
        //let mut block_count = 0;
        //for i in (0..tmap.len()).step_by(2) {
            //if tmap[i] == 0x00 && tmap[i+1] == 0x00 {
                //continue
            //}
            //if (tmap[i] == 0x00 && tmap[i+1] == 0x80) || (tmap[i] == 0x00 && tmap[i+1] == 0xC0) {
                //block_count += 1;
            //} else {
                //let block_index = u16::from_le_bytes([tmap[i], tmap[i+1]]) * 1024;
                //let block_data  = &raw[block_index as usize..block_index as usize + 1024];
                //disk.data[block_count].append(&mut block_data.to_vec());
            //}
        //}
    //}
}

#[derive(Debug)]
pub struct FileHeader {
    /// Name of file
    name:  String,
    /// Type of file
    kind:  FileType,
    /// Size of file in bytes
    size:  u32,
    /// Address of first block
    start: u32,
}

fn load_headers (raw: &[u8], max: usize) -> Vec<FileHeader> {
    let mut headers = vec![];
    // Read up to `max` FS records
    for entry in 0..max {
        match FileHeader::read(raw, entry * 24) {
            Some(header) => headers.push(header),
            // Empty file header means we've reached the end
            None => break
        }
    }
    headers
}

impl FileHeader {
    fn read (raw: &[u8], offset: usize) -> Option<Self> {
        if raw[offset] == 0x00 {
            return None
        } else {
            let head = &raw[offset..offset+24];
            Some(Self {
                name:  u8_to_string(&head[..12]),
                kind:  file_type(head[0x10]),
                size:  u32::from_le_bytes([head[0x11], head[0x12], head[0x13], 0x00]),
                start: u32::from_le_bytes([head[0x14], head[0x15], 0x00, 0x00]),
            })
        }
    }
}

fn load_table (raw: &[u8], start: usize, end: usize) -> Vec<FileTableEntry> {
    let table = &raw[start..end];
    let mut blocks = Vec::with_capacity(table.len() / 2);
    for address in (0..table.len()).step_by(2) {
        match (table[address], table[address+1]) {
            // reserved for system
            (0x00, 0x00) => {
                blocks.push(FileTableEntry::Free);
            },
            // reserved for system
            (0x00, 0x40) => {
                blocks.push(FileTableEntry::Reserved);
            },
            // reserved for 2nd file entry
            (0x00, 0x80) => {
                blocks.push(FileTableEntry::Reserved2);
            },
            // end of file
            (0x00, 0xc0) => {
                blocks.push(FileTableEntry::EOF);
            },
            // file continues at
            _ => {
                blocks.push(FileTableEntry::Next(
                    u16::from_le_bytes([table[address], table[address+1]])
                ))
            }
        }
    }
    blocks
}

#[derive(Debug)]
pub enum FileTableEntry {
    Free,
    Reserved,
    Reserved2,
    EOF,
    Next(u16)
}

#[derive(Debug)]
pub struct DiskImage<const M: DeviceModel> {
    pub raw:     Vec<u8>,
    pub label:   String,
    pub headers: Vec<FileHeader>,
    pub table:   Vec<FileTableEntry>,
    pub blocks:  Vec<[u8; 1024]>
}

impl<const M: DeviceModel> DiskImage<M> {

    /** Create a filesystem image. */
    pub fn new (
        raw:          Vec<u8>,
        label_start:  usize,
        header_start: usize,
        max_entries:  usize,
        fat_start:    usize,
        fat_end:      usize
    ) -> Self {
        Self {
            label:   u8_to_string(&raw[label_start..label_start+12]),
            headers: load_headers(&raw.as_slice()[header_start..], max_entries),
            table:   load_table(&raw.as_slice(), fat_start, fat_end),
            blocks:  as_blocks(&raw),
            raw,
        }
    }

    pub fn list_files (self) -> Self {
        println!("\nLabel: {}", self.label);
        println!("Files:");
        for (i, header) in self.headers.iter().enumerate() {
            println!("\n{: >4} {:<12} {:>8} bytes from block 0x{:04x}: {:?}", i, header.name, header.start, header.size, header.kind);
            let mut block_index = Some(header.start);
            while block_index.is_some() {
                let index = block_index.unwrap() as usize;
                let dump0 = self.blocks[index][0..256].into_braille_dump();
                let dump1 = self.blocks[index][256..512].into_braille_dump();
                let dump2 = self.blocks[index][512..768].into_braille_dump();
                let dump3 = self.blocks[index][768..1024].into_braille_dump();
                println!("     0x{:04x} {: >4}", index, &dump0);
                println!("            {: >4}", &dump1);
                println!("            {: >4}", &dump2);
                println!("            {: >4}", &dump3);
                block_index = match self.table[index] {
                    FileTableEntry::Next(index) => Some(index as u32),
                    _ => None
                };
            }
        }
        self
    }

    pub fn add_sample (self, name: &str, data: &[u8]) -> Self {
        self.add_file(name, FileType::S3000Sample)
    }

    pub fn add_file (mut self, name: &str, kind: FileType) -> Self {
        self.headers.push(FileHeader { name: name.into(), kind, size: 0, start: 0 });
        self
    }

    /*

    /** List all files in the filesystem. */
    pub fn list (&self) -> Vec<Sample> {
        let mut samples = vec![];
        for i in 0..self.head.len() {
            let head  = self.head[i];
            let name  = u8_to_string(&head[..0x0b+1]);
            let kind  = file_type(&head[0x10]);
            let size  = u32::from_le_bytes([head[0x11], head[0x12], head[0x13], 0x00]) as usize;
            let start = 0x1000 * u16::from_le_bytes([head[0x14], head[0x15]]) as usize;
            println!("\n{name} {kind:?} @{start:?} +{size:?}");
            let data  = &self.data[i];
            if kind == FileType::S3000Sample {
                println!("header id {:02x}", data[0x00]);
                println!("bandwidth {:02x}", data[0x01]);
                println!("pitch     {:02x}", data[0x02]);
                println!("valid     {:02x}", data[0x0f]);
                println!("loops     {:02x}", data[0x10]);
                println!("1st loop  {:02x}", data[0x11]);
                println!("play type {:02x}", data[0x13]);
                println!("tune cent {:02x}", data[0x14]);
                println!("tune semi {:02x}", data[0x15]);
                println!("offset    {:02x}", data[0x8c]);
                samples.push(Sample {
                    name:        name,
                    size:        size as u32,
                    data:        &[],
                    sample_rate: SampleRate::Hz22050,
                    loop_mode:   LoopMode::Normal,
                    tuning_semi: 0,
                    tuning_cent: 0,
                    length:      0
                })
            }
        }
        samples
    }

    */

}

pub trait DiskSamples {

    fn header_length () -> usize;

    fn validate_sample (data: &[u8]) -> (u8, u32) {
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
        (channels, sample_rate)
    }

    fn add_sample (&mut self, name: &str, data: &[u8]) -> &mut Self {
        let header_length = Self::header_length();
        let (channels, sample_rate) = Self::validate_sample(data);
        let contents = &data[44..];
        let length = (contents.len() as u32).to_le_bytes();
        let mut output: Vec<u8> = Vec::with_capacity(header_length + contents.len());
        output[0x00] = 0x03; // format (S3000)
        let channels = data[22];
        if channels != 1 {
            panic!("mono wavs only")
        }
        output[0x01] = channels; // channels
        output[0x02] = 0x3C;     // original pitch
        // name
        let name_akai = str_to_name(name);
        put(&mut output, 0x03, &name_akai[..12]);
        output[0x0f] = 0x80; // valid
        output[0x10] = 0x01; // loops
        output[0x11] = 0x00; // first loop
        output[0x12] = 0x00; // dummy
        output[0x13] = 0x02; // don't loop
        output[0x14] = 0x00; // tune cent
        output[0x15] = 0x00; // tune semi
        // data abs. start addr. (internal?)
        put(&mut output, 0x16, &[0x00, 0x04, 0x01, 0x00]);
        // set sample length
        put(&mut output, 0x1a, &length);
        // set sample start
        put(&mut output, 0x1e, &[0x00, 0x00, 0x00, 0x00]);
        // set sample end
        put(&mut output, 0x22, &length);
        // set sample rate
        put(&mut output, 0x8a, &sample_rate.to_le_bytes());
        // copy sample data
        put(&mut output, header_length, &contents);

        self.add_file(name_akai, &output)
    }

    fn add_file (&mut self, name_akai: Vec<u8>, data: &[u8]) -> &mut Self {
        let mut header = [0x00; 24];
        put(&mut header, 0x00, &name_akai[..12]);
        put(&mut header, 0x11, &data.len().to_le_bytes());
        put(&mut header, 0x11, &(0 as u32).to_le_bytes());
        self.write_header(header)
            .write_size(data.len() as u32)
            .write_data(data)
    }

    fn write_header (&mut self, header: [u8; 24]) -> &mut Self;
    fn write_size (&mut self, size: u32) -> &mut Self;
    fn write_data (&mut self, data: &[u8]) -> &mut Self;

}

/// Fill a buffer with content starting from offset.
fn put (buffer: &mut [u8], offset: usize, content: &[u8]) {
    for (index, value) in content.iter().enumerate() {
        if offset + index >= buffer.len() {
            break
        }
        buffer[offset + index] = *value
    }
}

fn as_blocks (data: &[u8]) -> Vec<[u8; 1024]> {
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
