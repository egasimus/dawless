use std::fs::File;

pub const CHARACTERS: [char; 41] = [
    '0','1','2','3','4','5','6','7','8','9',
    ' ',
    'A','B','C','D','E','F','G','H','I','J',
    'K','L','M','N','O','P','Q','R','S','T',
    'U','V','W','X','Y','Z','#','+','-','.',
];

#[derive(PartialEq, Eq)]
pub enum DeviceModel {
    S900,
    S2000,
    S3000
}

pub struct Device<const M: DeviceModel> {
}

impl<const M: DeviceModel> Device<M> {
    fn s900 () -> Device<{ DeviceModel::S900 }> {
        Device {}
    }
    fn s2000 () -> Device<{ DeviceModel::S2000 }> {
        Device {}
    }
    fn s3000 () -> Device<{ DeviceModel::S3000 }> {
        Device {}
    }
}

pub trait DeviceDisk<const M: DeviceModel, const N: usize> {
    fn create_blank_disk (&self) -> DiskImage<M, N>;
}

impl DeviceDisk<{ DeviceModel::S900 }, 819200> for Device<{ DeviceModel::S900 }>  {
    fn create_blank_disk (&self) -> DiskImage<{ DeviceModel::S900 }, 819200> {
        let mut image = DiskImage { data: [0; 819200] };
        image.blank();
        image
    }
}

impl DeviceDisk<{ DeviceModel::S2000 }, 1638400> for Device<{ DeviceModel::S2000 }> {
    fn create_blank_disk (&self) -> DiskImage<{ DeviceModel::S2000 }, 1638400> {
        let mut image = DiskImage { data: [0; 1638400] };
        image.blank();
        image
    }
}

impl DeviceDisk<{ DeviceModel::S3000 }, 1638400> for Device<{ DeviceModel::S3000 }> {
    fn create_blank_disk (&self) -> DiskImage<{ DeviceModel::S3000 }, 1638400> {
        let mut image = DiskImage { data: [0; 1638400] };
        image.blank();
        image
    }
}

pub struct DiskImage<const M: DeviceModel, const N: usize> {
    data: [u8; N]
}

pub trait DiskData<const N: usize>: Sized {
    fn data_mut (&mut self) -> &mut [u8; N];

    fn put_data <const L: usize> (&mut self, index: usize, section: &[u8; L]) -> usize {
        let length = section.len();
        self.data_mut()[index..index+length].copy_from_slice(section);
        index + length
    }

    fn blank (&mut self) {
        let index  = 0;
        let index  = self.put_data(index, &self.section_dp1());
        let index  = self.put_data(index, &self.section_dp2());
        let index  = self.put_data(index, &self.section_dp3());
        let index  = self.put_data(index, &self.section_dp4());
        let index  = self.put_data(index, &self.section_dp5());
        let _index = self.put_data(index, &self.section_dp6());
    }

    /** Disk header. */
    fn section_dp1 (&self) -> [u8; 24];

    /** Bunch of empty space. (what does it mean?) */
    fn section_dp2 (&self) -> [u8; 3166] {
        [0; 3166]
    }

    /** Volume name in AKAI format. (offset 4736) */
    fn section_dp3 (&self) -> [u8; 12] {
        [0x18, 0x19, 0x1E, 0x0A, 0x18, 0x0B, 0x17, 0x0F, 0x0E, 0x0A, 0x0A, 0x0A]
    }

    /** Not supported on S900, different on S2000 vs S3000. (what does it mean?) */
    fn section_dp4 (&self) -> [u8; 372];

    /** 512 possible directory entries. (offset 5120) */
    fn section_dp5 (&self) -> [u8; 12288] {
        [0; 12288]
    }

    /** 1583 * 1024 byte sectors for data. (offset 17408?) */
    fn section_dp6 (&self) -> [u8; 1620992] {
        [0; 1620992]
    }
}

impl DiskData<819200> for DiskImage<{ DeviceModel::S900 }, 819200> {
    fn data_mut (&mut self) -> &mut [u8; 819200] {
        &mut self.data
    }
    fn section_dp1 (&self) -> [u8; 24] {
        [
            0x0A, 0x0A, 0x0A, 0x0A, 0x0A, 0x0A, 0x0A, 0x0A,
            0x0A, 0x0A, 0x0A, 0x0A, 0x00, 0x00, 0x06, 0x0A,
            0xFF, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00
        ]
    }
    fn section_dp4 (&self) -> [u8; 372] {
        unreachable!("S900 images have no dp4 section")
    }
}

impl DiskData<1638400> for DiskImage<{ DeviceModel::S2000 }, 1638400> {
    fn data_mut (&mut self) -> &mut [u8; 1638400] {
        &mut self.data
    }
    fn section_dp1 (&self) -> [u8; 24] {
        [
            0x0A, 0x0A, 0x0A, 0x0A, 0x0A, 0x0A, 0x0A, 0x0A,
            0x0A, 0x0A, 0x0A, 0x0A, 0x00, 0x00, 0x06, 0x0A,
            0xFF, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x17
        ]
    }
    fn section_dp4 (&self) -> [u8; 372] {
        let mut data = [0; 372];
        const DP4_S2000: [u8; 14] = [
            0x00, 0x00, 0x00, 0x11, 0x00, 0x01, 0x01,
            0x23, 0x00, 0x00, 0x32, 0x09, 0x0C, 0xFF
        ];
        data[..DP4_S2000.len()].copy_from_slice(&DP4_S2000);
        data
    }
}

impl DiskData<1638400> for DiskImage<{ DeviceModel::S3000 }, 1638400> {
    fn data_mut (&mut self) -> &mut [u8; 1638400] {
        &mut self.data
    }
    fn section_dp1 (&self) -> [u8; 24] {
        [
            0x0A, 0x0A, 0x0A, 0x0A, 0x0A, 0x0A, 0x0A, 0x0A,
            0x0A, 0x0A, 0x0A, 0x0A, 0x00, 0x00, 0x06, 0x0A,
            0xFF, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x16
        ]
    }
    fn section_dp4 (&self) -> [u8; 372] {
        let mut data = [0; 372];
        const DP4_S3000: [u8; 14] = [
            0x00, 0x00, 0x32, 0x10, 0x00, 0x01, 0x01,
            0x00, 0x00, 0x00, 0x32, 0x09, 0x0C, 0xFF
        ];
        data[..DP4_S3000.len()].copy_from_slice(&DP4_S3000);
        data
    }
}

const BLOCK_SIZE: usize = 1024;
const MAX_BLOCKS: usize = 1536;
const FAT_HEADER: usize = 24;

struct DiskFiles<const N: usize> {
    model:  DeviceModel,
    blocks: [[u8; BLOCK_SIZE]; MAX_BLOCKS],
    head:   [[u8; FAT_HEADER]; N],
    size:   [u32; N],
    data:   [Vec<u8>; N],
    free:   usize
}

impl<const N: usize> DiskFiles<N> {

    /** Create a filesystem. */
    fn new (model: DeviceModel) -> Self {
        Self {
            model:  model,
            blocks: [[0; BLOCK_SIZE]; MAX_BLOCKS],
            head:   [[0; FAT_HEADER]; N],
            size:   [0; N],
            data:   array_init::array_init(|_| vec![]),
            free:   0
        }
    }

    /** Create a filesystem and populate it with data from a disk image. */
    fn load (model: DeviceModel, contents: &[u8]) -> Self {
        let mut fs = Self::new(model);
        fs.load_heads(contents);
        fs.load_tails(contents);
        fs
    }

    /// Reads the metadata and 1st block of each file in the disk image.
    /// Corresponds to 1st loop of s2kdie importimage().
    fn load_heads (&mut self, contents: &[u8]) -> &mut Self {
        let (_, _, data_offset, max_entries, max_blocks) = get_model_parameters(&self.model);
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
            head.copy_from_slice(&contents[entry_offset..24]);
            self.head[i] = head;
            self.size[i] = u32::from_le_bytes([head[17], head[18], head[19], 0x00]);
            self.data[i] = self.blocks[u16::from_le_bytes([head[20], head[21]]) as usize].into();
            last_block += self.size[i] / 1024;
            i += 1;
        }
        self.free = max_blocks - last_block as usize;
        self
    }

    /// Reads subsequent blocks (fragments) from the image.
    /// Corresponds to 2nd loop of s2kdie importimage()
    fn load_tails (&mut self, contents: &[u8]) -> &mut Self {
        let (startb, endb, _, _, _) = get_model_parameters(&self.model);
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
                let block_index = u16::from_le_bytes([tmap[i], tmap[i+1]]);
                let block_data  = &contents[block_index as usize..1024];
                self.data[block_count].append(&mut block_data.to_vec());
            }
        }
        self
    }

}

fn guess_model (contents: &[u8]) -> DeviceModel {
    let volname = &contents[4736..26];
    match volname[23] {
        0x00 => DeviceModel::S900,
        0x17 => DeviceModel::S2000,
        0x16 => DeviceModel::S3000,
        _ => panic!("could not determine device model from image data")
    }
}

fn get_model_parameters (model: &DeviceModel) -> (usize, usize, usize, usize, usize) {
    // Start of header
    let startb  = match model { DeviceModel::S900 => 1536, _ => 1570 };
    // End of header
    let endb    = match model { DeviceModel::S900 => 3136, _ => 3166 };
    // Offset of file table
    let offset  = match model { DeviceModel::S900 => 0,    _ => 5120 };
    // Number of entries in file table
    let entries = match model { DeviceModel::S900 => 64,   _ => 512  };
    // Free block(s?)
    let fb      = match model { DeviceModel::S900 => 796,  _ => 1583 };

    (startb, endb, offset, entries, fb)
}

pub enum DataTypes {
    Program = 240,
    Sample  = 243,
    Effects = 130,
    Multi   = 2378,
    OS      = 99,
    Deleted = 0
}
