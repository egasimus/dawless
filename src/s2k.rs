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
        DiskImage { data: [0; 819200] }.blank()
    }
}

impl DeviceDisk<{ DeviceModel::S2000 }, 1638400> for Device<{ DeviceModel::S2000 }> {
    fn create_blank_disk (&self) -> DiskImage<{ DeviceModel::S2000 }, 1638400> {
        DiskImage { data: [0; 1638400] }.blank()
    }
}

impl DeviceDisk<{ DeviceModel::S3000 }, 1638400> for Device<{ DeviceModel::S3000 }> {
    fn create_blank_disk (&self) -> DiskImage<{ DeviceModel::S3000 }, 1638400> {
        DiskImage { data: [0; 1638400] }.blank()
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

    fn blank (&mut self) -> Self {
        let index = 0;
        let index = self.put_data(index, &self.section_dp1());
        let index = self.put_data(index, &self.section_dp2());
        let index = self.put_data(index, &self.section_dp3());
        let index = self.put_data(index, &self.section_dp4());
        let index = self.put_data(index, &self.section_dp5());
        let index = self.put_data(index, &self.section_dp6());
        *self
    }

    /** Disk header. */
    fn section_dp1 (&mut self) -> [u8; 24];

    /** Bunch of empty space. (what does it mean?) */
    fn section_dp2 (&mut self) -> [u8; 3166] {
        [0; 3166]
    }

    /** Volume name in AKAI format. (offset 4736) */
    fn section_dp3 (&mut self) -> [u8; 12] {
        [0x18, 0x19, 0x1E, 0x0A, 0x18, 0x0B, 0x17, 0x0F, 0x0E, 0x0A, 0x0A, 0x0A]
    }

    /** Not supported on S900, different on S2000 vs S3000. (what does it mean?) */
    fn section_dp4 (&mut self) -> [u8; 372];

    /** 512 possible directory entries. (offset 5120) */
    fn section_dp5 (&mut self) -> [u8; 12288] {
        [0; 12288]
    }

    /** 1583 * 1024 byte sectors for data. (offset 17408?) */
    fn section_dp6 (&mut self) -> [u8; 1620992] {
        [0; 1620992]
    }
}

impl DiskData<819200> for DiskImage<{ DeviceModel::S900 }, 819200> {
    fn data_mut (&mut self) -> &mut [u8; 819200] {
        &mut self.data
    }
    fn section_dp1 (&mut self) -> [u8; 24] {
        [
            0x0A, 0x0A, 0x0A, 0x0A, 0x0A, 0x0A, 0x0A, 0x0A,
            0x0A, 0x0A, 0x0A, 0x0A, 0x00, 0x00, 0x06, 0x0A,
            0xFF, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00
        ]
    }
    fn section_dp4 (&mut self) -> [u8; 372] {
        unreachable!("S900 images have no dp4 section")
    }
}

impl DiskData<1638400> for DiskImage<{ DeviceModel::S2000 }, 1638400> {
    fn data_mut (&mut self) -> &mut [u8; 1638400] {
        &mut self.data
    }
    fn section_dp1 (&mut self) -> [u8; 24] {
        [
            0x0A, 0x0A, 0x0A, 0x0A, 0x0A, 0x0A, 0x0A, 0x0A,
            0x0A, 0x0A, 0x0A, 0x0A, 0x00, 0x00, 0x06, 0x0A,
            0xFF, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x17
        ]
    }
    fn section_dp4 (&mut self) -> [u8; 372] {
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
    fn section_dp1 (&mut self) -> [u8; 24] {
        [
            0x0A, 0x0A, 0x0A, 0x0A, 0x0A, 0x0A, 0x0A, 0x0A,
            0x0A, 0x0A, 0x0A, 0x0A, 0x00, 0x00, 0x06, 0x0A,
            0xFF, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x16
        ]
    }
    fn section_dp4 (&mut self) -> [u8; 372] {
        let mut data = [0; 372];
        const DP4_S3000: [u8; 14] = [
            0x00, 0x00, 0x32, 0x10, 0x00, 0x01, 0x01,
            0x00, 0x00, 0x00, 0x32, 0x09, 0x0C, 0xFF
        ];
        data[..DP4_S3000.len()].copy_from_slice(&DP4_S3000);
        data
    }
}

fn import_image <const N: usize> (contents: &[u8; N]) {

    let volname = &contents[4736..26];

    let model = match volname[23] {
        0x00 => DeviceModel::S900,
        0x17 => DeviceModel::S2000,
        0x16 => DeviceModel::S3000,
        _ => panic!("could not determine device model from image data")
    };

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

    let fat;
    let file;
    let freeblocks;
    let fsize;
    let ttype;
    let fs;

    if contents[offset] != 0x00 {

        let mut cl    = 0;

        // Block offset?
        let mut block = 0;

        let tmap = &contents[startb..endb-startb];

        blocks = str_split($contents,1024);

        for i in 0..entries {
            let entry_offset = offset + i * 24;
            if (contents[entry_offset] == 0x00) {
                break
            }

            let fileentry = &contents[entry_offset..24];

            fat[cl] = fileentry;

            sb = hexdec(
                str_pad(dechex(ord($fileentry[21])),2,"0",STR_PAD_LEFT) .
                str_pad(dechex(ord($fileentry[20])),2,"0",STR_PAD_LEFT)
            );

            fsize[cl] = hexdec(
                str_pad(dechex(ord($fileentry[19])),2,"0",STR_PAD_LEFT) . 
                str_pad(dechex(ord($fileentry[18])),2,"0",STR_PAD_LEFT) . 
                str_pad(dechex(ord($fileentry[17])),2,"0",STR_PAD_LEFT)
            );

            file[cl] = blocks[sb];
  
            block = block + fsize[cl] / 1024;

            cl += 1;
        }

        let mut bc = 0;
        for i in [0..tmap.len()].iter().step_by(2) {
          if bc >= sizeof($fat) {
              continue
          }
          if tmap[i] == 0x00 && tmap[i+1] == 0x00 {
            continue;
          }
          if (tmap[i] == 0x00 && tmap[i+1] == 0x80) || (tmap[i] == 0x00 && tmap[i+1] == 0xC0) {
            bc += 1;
          } else {
            file[bc].append(
                blocks[hexdec(str_pad(dechex(ord($tmap[$i+1])),2,"0",STR_PAD_LEFT) .
                str_pad(dechex(ord($tmap[$i])),2,"0",STR_PAD_LEFT))]
            );
          }
        }
        freeblocks = fb - block;
    }

}


pub enum DataTypes {
    Program = 240,
    Sample  = 243,
    Effects = 130,
    Multi   = 2378,
    OS      = 99,
    Deleted = 0
}

pub fn ascii2akai (inp: Vec<u8>) -> Vec<u8> {}

pub fn akai2ascii (inp: vec<u8>) -> Vec<u8> {}
