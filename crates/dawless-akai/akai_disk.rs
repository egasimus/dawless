use crate::*;

pub trait DeviceDisk<const M: DeviceModel> {

    /** Create and format an empty disk. */
    fn blank_disk (&self) -> DiskImage<M> {
        self.load_disk(write_fixed_sections(&M, Vec::with_capacity(disk_capacity(&M))))
    }

    /** Read the files from a disk image. */
    fn load_disk (&self, raw: Vec<u8>) -> DiskImage<M> {
        DiskImage::new(&M, raw)
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
        model: &DeviceModel,
        raw:   Vec<u8>,
    ) -> Self {
        let label_start  = 0x1280;
        Self {
            label:   u8_to_string(&raw[label_start..label_start+12]),
            headers: FileRecord::read_all(&model, &raw.as_slice()),
            table:   read_block_table(&model, &raw.as_slice()),
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
        self.add_file(
            name.into(),
            FileType::S3000Sample,
            &Sample::serialize(name, data)
        )
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
        let data = write_fixed_sections(&M, Vec::with_capacity(1638400));
        let data = FileRecord::write_all(&M, data, &self.headers);
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
