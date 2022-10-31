use crate::*;

pub trait DeviceDisk<const M: DeviceModel> {

    /** Create and format an empty disk. */
    fn blank_disk (&self) -> DiskImage<M> {
        self.load_disk(
            write_fixed_sections(&M, Vec::with_capacity(Self::disk_size()))
        )
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

    fn max_entries () -> usize;

    fn header_start () -> usize;

    fn max_blocks () -> usize;

    fn fat_start () -> usize;

    fn fat_end () -> usize;

    fn label_start () -> usize {
        4736
    }

}

#[derive(Debug)]
pub struct DiskImage<const M: DeviceModel> {
    pub label:   String,
    pub headers: Vec<FileRecord>,
    pub table:   Vec<BlockRecord>,
    pub blocks:  Vec<BlockData>
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
            headers: load_file_headers(&raw.as_slice()[header_start..], max_entries),
            table:   load_block_table(&raw.as_slice(), fat_start, fat_end),
            blocks:  as_blocks(&raw),
        }
    }

    pub fn list_files (self) -> Self {
        println!("\nLabel: {}", self.label);
        println!("Files:");
        for (i, header) in self.headers.iter().enumerate() {
            println!("\n{: >4} {:<12} {:>8} bytes from block 0x{:04x} is a {:?}", i, header.name, header.size, header.start, header.kind);
            let data = self.read_file(header.start, header.size);
            //let mut block_index = Some(header.start);
            //while block_index.is_some() {
                //let index = block_index.unwrap() as usize;
                //let dump0 = self.blocks[index][0..256].into_braille_dump();
                //let dump1 = self.blocks[index][256..512].into_braille_dump();
                //let dump2 = self.blocks[index][512..768].into_braille_dump();
                //let dump3 = self.blocks[index][768..1024].into_braille_dump();
                //println!("     0x{:04x} {: >4}", index, &dump0);
                //println!("            {: >4}", &dump1);
                //println!("            {: >4}", &dump2);
                //println!("            {: >4}", &dump3);
                //block_index = match self.table[index] {
                    //BlockRecord::Next(index) => Some(index as u16),
                    //_ => None
                //};
            //}
        }
        self
    }

    pub fn read_file (&self, mut start: u16, size: u32) -> Vec<u8> {
        let mut data      = vec![0u8; size as usize];
        let mut remaining = size as usize;
        let mut index     = 0;
        while remaining > 0 {
            //println!("     block {}, remaining: {}, next: {:?}", start, remaining, self.table[start as usize]);
            let block = self.blocks[start as usize];
            let count = put_vec(&mut data, index, &block[..usize::min(remaining, 1024)]);
            index += count;
            remaining = remaining.saturating_sub(count);
            match self.table[start as usize] {
                BlockRecord::Next(block) => start = block,
                _ => break
            }
        }
        data
    }

    pub fn add_sample (self, name: &str, data: &[u8]) -> Self {
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
        let length = (contents.len() as u32).to_le_bytes();
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

        self.add_file(name.into(), FileType::S3000Sample, &output)
    }

    pub fn add_file (mut self, name: &str, kind: FileType, data: &[u8]) -> Self {
        let start = 0x03;
        let head = FileRecord { name: name.into(), kind, size: data.len() as u32, start };
        self.headers.push(head);
        self.write_blocks(start, data)
    }

    fn write_blocks (mut self, mut index: u16, data: &[u8]) -> Self {
        let mut new_blocks: std::collections::VecDeque<BlockData> = as_blocks(data).into();
        while let Some(block) = new_blocks.pop_front() {
            self.blocks[index as usize] = block;
            match self.table.iter().position(|x| *x == BlockRecord::Free) {
                Some(found) => {
                    self.table[index as usize] = BlockRecord::Next(found as u16);
                    index = found as u16
                },
                None => panic!("ran out of free blocks")
            }
        }
        self.table[index as usize] = BlockRecord::EOF;
        self
    }

    pub fn write_disk (self) -> Vec<u8> {
        let data = vec![0x00; 1638400];
        data
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
        put_vec_max(max, &mut output, 0x03, &name_akai[..12]);
        output[0x0f] = 0x80; // valid
        output[0x10] = 0x01; // loops
        output[0x11] = 0x00; // first loop
        output[0x12] = 0x00; // dummy
        output[0x13] = 0x02; // don't loop
        output[0x14] = 0x00; // tune cent
        output[0x15] = 0x00; // tune semi
        // data abs. start addr. (internal?)
        put_vec_max(max, &mut output, 0x16, &[0x00, 0x04, 0x01, 0x00]);
        // set sample length
        put_vec_max(max, &mut output, 0x1a, &length);
        // set sample start
        put_vec_max(max, &mut output, 0x1e, &[0x00, 0x00, 0x00, 0x00]);
        // set sample end
        put_vec_max(max, &mut output, 0x22, &length);
        // set sample rate
        put_vec_max(max, &mut output, 0x8a, &sample_rate.to_le_bytes());
        // copy sample data
        put_vec_max(max, &mut output, header_length, &contents);

        self.add_file(name_akai, &output)
    }

    fn add_file (&mut self, name_akai: Vec<u8>, data: &[u8]) -> &mut Self {
        let mut header = [0x00; 24];
        put(&mut header, 0x00, &name_akai[..12]);
        put(&mut header, 0x11, &data.len().to_le_bytes());
        put(&mut header, 0x11, &(0 as u32).to_le_bytes());
        self
        //self.write_header(header)
            //.write_size(data.len() as u32)
            //.write_data(data)
    }

}

/// Split a slice into blocks of 1024 bytes
fn as_blocks (data: &[u8]) -> Vec<BlockData> {
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
