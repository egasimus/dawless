use crate::*;

pub trait DeviceDisk<const M: DeviceModel> {

    /** Create and format an empty disk. */
    fn blank_disk (&self) -> DiskImage<M> {
        let mut raw = Vec::with_capacity(Self::capacity());
        raw.extend_from_slice(&self.section1());
        raw.extend_from_slice(&self.section2());
        raw.extend_from_slice(&self.section3());
        raw.extend_from_slice(&self.section4());
        raw.extend_from_slice(&self.section5());
        raw.extend_from_slice(&self.section6());
        DiskImage::new(raw, Self::fat_max_entries())
    }

    fn capacity () -> usize;

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

    /** Read the files from a disk image. */
    fn load_disk (&self, raw: Vec<u8>) -> DiskImage<M> {
        let mut disk = DiskImage::new(raw, Self::fat_max_entries());
        let label_offset = Self::fat_label_offset();
        disk.label = u8_to_string(&disk.raw[label_offset..label_offset+12]);
        Self::load_disk_head(&mut disk);
        Self::load_disk_body(&mut disk);
        disk
    }

    fn fat_max_entries () -> usize;

    fn fat_label_offset () -> usize {
        4736
    }

    /// Reads the metadata and 1st block of each file in the disk image.
    /// Corresponds to 1st loop of s2kdie importimage().
    fn load_disk_head (disk: &mut DiskImage<M>) {
        let raw = disk.raw.as_slice();
        let data_offset = Self::fat_offset();
        let max_entries = Self::fat_max_entries();
        let max_blocks  = Self::fat_max_blocks();
        // Used to determine number of remaining blocks
        let mut last_block = 0;
        // Read up to `max_entries` FS records
        let mut i = 0;
        while i < max_entries {
            let entry_offset = data_offset + i * 24;
            // If we've reached an empty entry we're past the end
            if raw[entry_offset] == 0x00 {
                break
            }
            let mut head: [u8; 24] = [0; 24];
            head.copy_from_slice(&raw[entry_offset..entry_offset+24]);
            disk.head.push(head);
            let size = u32::from_le_bytes([head[17], head[18], head[19], 0x00]);
            disk.size.push(size);
            let block_index = u32::from_le_bytes([head[20], head[21], 0x00, 0x00]);
            let block_start = (block_index * 1024) as usize;
            let block_end   = ((block_index + 1) * 1024) as usize;
            let block_data  = &raw[block_start..block_end];
            disk.data.push(block_data.to_vec());
            last_block += size / 1024;
            i += 1;
        }
        disk.free = max_blocks - last_block as usize;
    }

    fn fat_offset () -> usize;

    fn fat_max_blocks  () -> usize;

    /// Reads subsequent blocks (fragments) from the image.
    /// Corresponds to 2nd loop of s2kdie importimage()
    fn load_disk_body (disk: &mut DiskImage<M>) {
        let startb = Self::fat_start();
        let endb   = Self::fat_end();
        let raw    = disk.raw.as_slice();
        let tmap   = &raw[startb..endb-startb];
        let mut block_count = 0;
        for i in (0..tmap.len()).step_by(2) {
            if tmap[i] == 0x00 && tmap[i+1] == 0x00 {
                continue
            }
            if (tmap[i] == 0x00 && tmap[i+1] == 0x80) || (tmap[i] == 0x00 && tmap[i+1] == 0xC0) {
                block_count += 1;
            } else {
                let block_index = u16::from_le_bytes([tmap[i], tmap[i+1]]) * 1024;
                let block_data  = &raw[block_index as usize..block_index as usize + 1024];
                disk.data[block_count].append(&mut block_data.to_vec());
            }
        }
    }

    fn fat_start () -> usize;

    fn fat_end () -> usize;
}

#[derive(Debug)]
pub struct DiskImage<const M: DeviceModel> {
    pub(crate) raw:   Vec<u8>,
    pub(crate) label: String,
    pub(crate) head:  Vec<[u8; 24]>,
    pub(crate) size:  Vec<u32>,
    pub(crate) data:  Vec<Vec<u8>>,
    pub(crate) free:  usize
}

impl<const M: DeviceModel> DiskImage<M> {

    /** Create a filesystem. */
    pub fn new (raw: Vec<u8>, max_entries: usize) -> Self {
        Self {
            raw,
            label: String::new(),
            head:  Vec::with_capacity(max_entries),
            size:  Vec::with_capacity(max_entries),
            data:  Vec::with_capacity(max_entries),
            free:  0
        }
    }

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

}

pub trait DiskSamples {

    fn header_length () -> usize;

    fn add_sample (&mut self, name: &str, data: &[u8]) -> &mut Self {

        let header_length = Self::header_length();

        let compression = data[20];
        if compression != 1 {
            panic!("uncompressed wavs only")
        }

        let samplerate = u32::from_le_bytes([data[24], data[25], data[26], data[27]]);
        if samplerate != 44100 {
            panic!("44.1kHz wavs only")
        }

        let bitrate = data[34];
        if bitrate != 16 {
            panic!("16-bit wavs only")
        }

        let contents = &data[44..];
        let mut output: Vec<u8> = Vec::with_capacity(header_length + contents.len());

        output[0x00] = 0x03; // format (S3000)

        let channels = data[22];
        if channels != 1 {
            panic!("mono wavs only")
        }
        output[0x01] = channels; // channels
        output[0x02] = 0x3C;     // original pitch
        
        // name
        put(&mut output, 0x03, &str_to_name(name)[..12]);

        output[0x0f] = 0x80; // valid
        output[0x10] = 0x01; // loops
        output[0x11] = 0x00; // first loop
        output[0x12] = 0x00; // dummy
        output[0x13] = 0x02; // don't loop
        output[0x14] = 0x00; // tune cent
        output[0x15] = 0x00; // tune semi

        // data abs. start addr. (internal?)
        put(&mut output, 0x16, &[0x00, 0x04, 0x01, 0x00]);

        let length = (contents.len() as u32).to_le_bytes();

        // set sample length
        put(&mut output, 0x1a, &length);

        // set sample start
        put(&mut output, 0x1e, &[0x00, 0x00, 0x00, 0x00]);

        // set sample end
        put(&mut output, 0x22, &length);

        // set sample rate
        put(&mut output, 0x8a, &(44100 as u16).to_le_bytes());

        // copy sample data
        put(&mut output, header_length, &contents);

        self
    }

}

fn put (output: &mut [u8], offset: usize, data: &[u8]) {
    for (index, value) in data.iter().enumerate() {
        output[offset + index] = *value
    }
}
