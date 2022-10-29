
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

pub struct Device<const M: DeviceModel> {}
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
    fn create_blank_disk (&self) -> DiskImage<M, N> {
        DiskImage::blank()
    }
}
impl DeviceDisk<{ DeviceModel::S900  }, 819200>  for Device<{ DeviceModel::S900 }>  {}
impl DeviceDisk<{ DeviceModel::S2000 }, 1638400> for Device<{ DeviceModel::S2000 }> {}
impl DeviceDisk<{ DeviceModel::S3000 }, 1638400> for Device<{ DeviceModel::S3000 }> {}
pub struct DiskImage<const M: DeviceModel, const N: usize> {
    data: [u8; N]
}
pub trait DiskSections {
    /** Disk header. */
    fn dp1 () -> [u8; 24];
    /** Bunch of empty space. (what does it mean?) */
    fn dp2 () -> [u8; 3166] {
        [0; 3166]
    }
    /** Volume name in AKAI format. (offset 4736) */
    fn dp3 () -> [u8; 12] {
        [0x18, 0x19, 0x1E, 0x0A, 0x18, 0x0B, 0x17, 0x0F, 0x0E, 0x0A, 0x0A, 0x0A]
    }
    /** Not supported on S900, different on S2000 vs S3000. (what does it mean?) */
    fn dp4 () -> [u8; 372];
    /** 512 possible directory entries. (offset 5120) */
    fn dp5 () -> [u8; 12288] {
        [0; 12288]
    }
    /** 1583 * 1024 byte sectors for data. (offset 17408?) */
    fn dp6 () -> [u8; 1620992] {
        [0; 1620992]
    }
}
impl DiskSections for DiskImage<{ DeviceModel::S900 }, 819200> {
    fn dp1 () -> [u8; 24] {
        [
            0x0A, 0x0A, 0x0A, 0x0A, 0x0A, 0x0A, 0x0A, 0x0A,
            0x0A, 0x0A, 0x0A, 0x0A, 0x00, 0x00, 0x06, 0x0A,
            0xFF, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00
        ]
    }
    fn dp4 () -> [u8; 372] {
        unreachable!("S900 images have no dp4 section")
    }
}
impl DiskSections for DiskImage<{ DeviceModel::S2000 }, 1638400> {
    fn dp1 () -> [u8; 24] {
        [
            0x0A, 0x0A, 0x0A, 0x0A, 0x0A, 0x0A, 0x0A, 0x0A,
            0x0A, 0x0A, 0x0A, 0x0A, 0x00, 0x00, 0x06, 0x0A,
            0xFF, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x17
        ]
    }
    fn dp4 () -> [u8; 372] {
        let mut data = [0; 372];
        const DP4_S2000: [u8; 14] = [
            0x00, 0x00, 0x00, 0x11, 0x00, 0x01, 0x01,
            0x23, 0x00, 0x00, 0x32, 0x09, 0x0C, 0xFF
        ];
        let (dest, _) = data.split_at_mut(DP4_S2000.len());
        dest.copy_from_slice(&DP4_S2000);
        data
    }
}
impl DiskSections for DiskImage<{ DeviceModel::S3000 }, 1638400> {
    fn dp1 () -> [u8; 24] {
        [
            0x0A, 0x0A, 0x0A, 0x0A, 0x0A, 0x0A, 0x0A, 0x0A,
            0x0A, 0x0A, 0x0A, 0x0A, 0x00, 0x00, 0x06, 0x0A,
            0xFF, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x16
        ]
    }
    fn dp4 () -> [u8; 372] {
        let mut data = [0; 372];
        const DP4_S3000: [u8; 14] = [
            0x00, 0x00, 0x32, 0x10, 0x00, 0x01, 0x01,
            0x00, 0x00, 0x00, 0x32, 0x09, 0x0C, 0xFF
        ];
        let (dest, _) = data.split_at_mut(DP4_S3000.len());
        dest.copy_from_slice(&DP4_S3000);
        data
    }
}

impl<const M: DeviceModel, const N: usize> DiskImage<M, N> {
    fn import (model: DeviceModel, path: &File) -> Self {
        let image = Self::create(model);
        return image
    }
    fn export (&self, path: &Path) {
    }

    fn dp1 () -> Vec<u8> {
      global $type,$dp1;
      if ($type == "S2000") { $id = chr(17); }
      if ($type == "S3000") { $id = chr(16); }
      if ($type == "S900") { $id = chr(0); }
      $dp1 =            "\x0A\x0A\x0A\x0A\x0A\x0A\x0A\x0A\x0A\x0A\x0A\x0A\x00\x00\x06\x0A\xFF\x00\x00\x04\x00\x00\x00" . $id .
             str_repeat("\x0A\x0A\x0A\x0A\x0A\x0A\x0A\x0A\x0A\x0A\x0A\x0A\x00\x00\x06\x0A\x00\x00\x00\x04\x00\x00\x00"  . $id,63) . 
             str_repeat("\x00\x40",17);
    }

    fn dp2 () -> Vec<u8> {
        return str_repeat("\x00",3166); 
    }

    /** Volume name in AKAI format. (offset 4736) */
    fn dp3_volname () -> Vec<u8> {
        return "\x18\x19\x1E\x0A\x18\x0B\x17\x0F\x0E\x0A\x0A\x0A";
    }

    fn dp4_s2000 () -> Vec<u8> {
        return "\x00\x00\x00\x11\x00\x01\x01\x23\x00\x00\x32\x09\x0C\xFF" . str_repeat("\x00",358);
    }

    fn dp4_s3000 () -> Vec<u8> {
        "\x00\x00\x32\x10\x00\x01\x01\x00\x00\x00\x32\x09\x0C\xFF" . str_repeat("\x00",358);
    }

    /** (dp5) 512 possible directory entries starting at offset 5120. */
    fn dp5_dir_entries () -> {
        return str_repeat(
            "\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00",
            512
        );
    }

    /** (dp6) 1583 * 1024 byte sectors for data. */
    fn dp6_disk_sectors () -> {
        return str_repeat(
            str_repeat("\x00",1024),
            1583
        );
    }

    fn import () -> Vec<u8> {
        let data = vec![];
        return data
    }

    fn defrag () -> Vec<u8> {
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
