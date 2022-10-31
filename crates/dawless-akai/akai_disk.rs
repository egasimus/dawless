use crate::*;

pub trait DeviceDisk<const M: DeviceModel> {

    /** Create and format an empty disk. */
    fn blank_disk (&self) -> Filesystem<M> {
        self.load_disk(write_fixed_sections(&M, Vec::with_capacity(disk_capacity(&M))))
    }

    /** Read the files from a disk image. */
    fn load_disk (&self, raw: Vec<u8>) -> Filesystem<M> {
        Filesystem::new(raw)
    }

}

#[derive(Debug)]
pub struct Filesystem<const M: DeviceModel> {
    pub label:   String,
    pub headers: Vec<FileRecord>,
    pub table:   Vec<BlockRecord>,
    pub blocks:  Vec<BlockData>
}

impl<const M: DeviceModel> Filesystem<M> {

    /** Create a filesystem image. */
    pub fn new (raw: Vec<u8>) -> Self {
        let label   = u8_to_string(&raw[0x1280..0x1280+12]);
        let headers = FileRecord::read_all::<M>(&raw.as_slice());
        let table   = read_block_table::<M>(&raw.as_slice());
        let blocks  = as_blocks(&raw);
        Self { label, headers, table, blocks }
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
        let start = self.find_free_block();
        let head = FileRecord { name: name.into(), kind, size: data.len() as u32, start };
        self.headers.push(head);
        self.put_blocks(start, data)
    }

    fn find_free_block (&self) -> BlockIndex {
        // start from block 17 (0x11 at addr 0x4400 - first data block in image)
        for i in 0x11..max_blocks(&M) {
            if self.table[i] == BlockRecord::Free {
                return i as BlockIndex
            }
        }
        panic!("no free block found")
    }

    fn put_blocks (mut self, mut index: BlockIndex, data: &[u8]) -> Self {
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
        let data = FileRecord::write_all::<M>(data, &self.headers);
        let data = write_block_table::<M>(data, &self.table);
        let data = write_blocks::<M>(data, &self.blocks);
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
