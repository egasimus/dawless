use crate::*;

pub struct DiskImage<const M: DeviceModel, const N: usize> {
    pub data: Vec<u8>
}

pub trait DeviceDisk<const M: DeviceModel, const N: usize> {

    fn disk (&self) -> DiskImage<M, N>;

    fn load (&self, raw: &Vec<u8>) -> DiskFiles<M, N> {
        DiskFiles::load(raw.as_slice())
    }

}

pub trait DiskBlank<const N: usize>: Sized {
    fn data_mut (&mut self) -> &mut Vec<u8>;

    fn put_data <const L: usize> (&mut self, index: usize, section: &[u8; L]) -> usize {
        let length = section.len();
        self.data_mut()[index..index+length].copy_from_slice(section);
        index + length
    }

    fn blank (&mut self) {
        let index  = 0;
        let index  = self.put_data(index, &self.section1());
        let index  = self.put_data(index, &self.section2());
        let index  = self.put_data(index, &self.section3());
        let index  = self.put_data(index, &self.section4());
        let index  = self.put_data(index, &self.section5());
        let _index = self.put_data(index, &self.section6());
    }

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
}

#[derive(Debug)]
pub struct DiskFiles<const M: DeviceModel, const N: usize> {
    label: String,
    head:  Vec<[u8; 24]>,
    size:  Vec<u32>,
    data:  Vec<Vec<u8>>,
    free:  usize
}

impl<const M: DeviceModel, const N: usize> DiskFiles<M, N> {

    /** Create a filesystem. */
    fn new () -> Self {
        Self {
            label: String::new(),
            head:  Vec::with_capacity(N),
            size:  Vec::with_capacity(N),
            data:  Vec::with_capacity(N),
            free:  0
        }
    }

    /** Create a filesystem and populate it with data from a disk image. */
    pub fn load (contents: &[u8]) -> Self {
        let mut fs = Self::new();
        fs.label = u8_to_string(&contents[4736..4736+12]);
        fs.load_heads(contents);
        fs.load_bodies(contents);
        fs
    }

    /// Reads the metadata and 1st block of each file in the disk image.
    /// Corresponds to 1st loop of s2kdie importimage().
    fn load_heads (&mut self, contents: &[u8]) -> &mut Self {
        let (_, _, data_offset, max_entries, max_blocks) = get_model_parameters(&M);
        // Used to determine number of remaining blocks
        let mut last_block = 0;
        // Read up to `max_entries` FS records
        let mut i = 0;
        while i < max_entries {
            let entry_offset = data_offset + i * 24;
            // If we've reached an empty entry we're past the end
            if contents[entry_offset] == 0x00 {
                break
            }
            let mut head: [u8; 24] = [0; 24];
            head.copy_from_slice(&contents[entry_offset..entry_offset+24]);
            self.head.push(head);
            let size = u32::from_le_bytes([head[17], head[18], head[19], 0x00]);
            self.size.push(size);
            let block_index = u32::from_le_bytes([head[20], head[21], 0x00, 0x00]);
            let block_start = (block_index * 1024) as usize;
            let block_end   = ((block_index + 1) * 1024) as usize;
            println!("bi={block_index}");
            let block_data  = &contents[block_start..block_end];
            self.data.push(block_data.to_vec());
            last_block += size / 1024;
            i += 1;
        }
        self.free = max_blocks - last_block as usize;
        self
    }

    /// Reads subsequent blocks (fragments) from the image.
    /// Corresponds to 2nd loop of s2kdie importimage()
    fn load_bodies (&mut self, contents: &[u8]) -> &mut Self {
        let (startb, endb, _, _, _) = get_model_parameters(&M);
        // Map of fragments
        let tmap = &contents[startb..endb-startb];
        let mut block_count = 0;
        for i in (0..tmap.len()).step_by(2) {
            if tmap[i] == 0x00 && tmap[i+1] == 0x00 {
                continue
            }
            if (tmap[i] == 0x00 && tmap[i+1] == 0x80) || (tmap[i] == 0x00 && tmap[i+1] == 0xC0) {
                block_count += 1;
            } else {
                let block_index = u16::from_le_bytes([tmap[i], tmap[i+1]]) * 1024;
                let block_data  = &contents[block_index as usize..block_index as usize + 1024];
                self.data[block_count].append(&mut block_data.to_vec());
            }
        }
        self
    }

    pub fn list (&self, raw: &[u8]) -> Vec<Sample> {
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
                    size:        0,
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
