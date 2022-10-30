use crate::*;

const BLOCK_SIZE: usize = 1024;
const MAX_BLOCKS: usize = 1536;
const HEADER_LEN: usize = 24;
const LABEL_SIZE: usize = 26;

#[derive(Debug)]
pub struct DiskFiles<const M: DeviceModel, const N: usize> {
    label: String,
    head:  Vec<[u8; HEADER_LEN]>,
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
            let block_index = u16::from_le_bytes([head[20], head[21]]);
            let block_data  = &contents[block_index as usize..1024];
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

    pub fn list (&self) -> Vec<Sample> {
        let mut samples = vec![];
        for i in 0..self.head.len() {
            let head = self.head[i];
            let size = self.size[i];
            let data = &self.data[i];
            samples.push(Sample {
                name:        u8_to_string(&head[..12]),
                sample_rate: SampleRate::Hz22050,
                loop_mode:   LoopMode::Normal,
                tuning_semi: 0,
                tuning_cent: 0,
                length:      0
            })
        }
        samples
    }

}
