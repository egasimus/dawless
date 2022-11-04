use crate::*;

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

pub trait DeviceDisk<const M: DeviceModel> {
    /** Create and format an empty disk. */
    fn blank_disk (&self) -> Filesystem<M> {
        self.load_disk(&format::<M>())
    }
    /** Read the files from a disk image. */
    fn load_disk (&self, raw: &Vec<u8>) -> Filesystem<M> {
        Filesystem::new(raw.clone())
    }
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
