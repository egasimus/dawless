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
