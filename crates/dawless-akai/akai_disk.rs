use crate::*;

pub trait DeviceDisk<const M: DeviceModel> {

    /** Create and format an empty disk. */
    fn blank_disk (&self) -> DiskImage<M> {
        let mut image = DiskImage { data: Vec::with_capacity(Self::capacity()) };
        let index  = 0;
        let index  = image.put_data(index, &self.section1());
        let index  = image.put_data(index, &self.section2());
        let index  = image.put_data(index, &self.section3());
        let index  = image.put_data(index, &self.section4());
        let index  = image.put_data(index, &self.section5());
        let _index = image.put_data(index, &self.section6());
        image
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
    fn load_disk (&self, raw: &Vec<u8>) -> DiskFiles<M> {
        let mut disk = DiskFiles::new(Self::fat_max_entries());
        let label_offset = Self::fat_label_offset();
        disk.label = u8_to_string(&raw[label_offset..label_offset+12]);
        Self::load_disk_head(&mut disk, raw);
        Self::load_disk_body(&mut disk, raw);
        disk
    }

    fn fat_label_offset () -> usize {
        4736
    }

    /// Reads the metadata and 1st block of each file in the disk image.
    /// Corresponds to 1st loop of s2kdie importimage().
    fn load_disk_head (disk: &mut DiskFiles<M>, raw: &[u8]) {
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

    fn fat_max_entries () -> usize;

    fn fat_max_blocks  () -> usize;

    /// Reads subsequent blocks (fragments) from the image.
    /// Corresponds to 2nd loop of s2kdie importimage()
    fn load_disk_body (disk: &mut DiskFiles<M>, contents: &[u8]) {
        let startb = Self::fat_start();
        let endb   = Self::fat_end();
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
                disk.data[block_count].append(&mut block_data.to_vec());
            }
        }
    }

    fn fat_start () -> usize;

    fn fat_end () -> usize;
}

pub struct DiskImage<const M: DeviceModel> {
    pub data: Vec<u8>
}

impl<const M: DeviceModel> DiskImage<M> {
    fn put_data <const L: usize> (&mut self, index: usize, section: &[u8; L]) -> usize {
        let length = section.len();
        self.data[index..index+length].copy_from_slice(section);
        index + length
    }
}

#[derive(Debug)]
pub struct DiskFiles<const M: DeviceModel> {
    label: String,
    head:  Vec<[u8; 24]>,
    size:  Vec<u32>,
    data:  Vec<Vec<u8>>,
    free:  usize
}

impl<const M: DeviceModel> DiskFiles<M> {

    /** Create a filesystem. */
    fn new (capacity: usize) -> Self {
        Self {
            label: String::new(),
            head:  Vec::with_capacity(capacity),
            size:  Vec::with_capacity(capacity),
            data:  Vec::with_capacity(capacity),
            free:  0
        }
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
