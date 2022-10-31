use crate::*;

pub struct Device<const M: DeviceModel>;

#[derive(PartialEq, Eq)]
pub enum DeviceModel {
    S900,
    S2000,
    S3000
}

pub fn akai_s900 () -> Device<{ DeviceModel::S900 }> {
    Device
}

pub fn akai_s2000 () -> Device<{ DeviceModel::S2000 }> {
    Device
}

pub fn akai_s3000 () -> Device<{ DeviceModel::S3000 }> {
    Device
}

/// These sections are written verbatim when creating a disk image.
pub fn write_fixed_sections (model: &DeviceModel, mut raw: Vec<u8>) -> Vec<u8> {

    let section0: &[u8; 23] = &[
        0x0A, 0x0A, 0x0A, 0x0A, 0x0A, 0x0A, 0x0A, 0x0A,
        0x0A, 0x0A, 0x0A, 0x0A, 0x00, 0x00, 0x06, 0x0A,
        0xFF, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00
    ];
    raw.extend_from_slice(section0);

    // 0x0000: Disk header
    let section1: &[u8; 1] = &match model {
        DeviceModel::S900  => [0x00],
        DeviceModel::S2000 => [0x17],
        DeviceModel::S3000 => [0x16]
    };
    raw.extend_from_slice(section1);

    // Bunch of empty space. (what does it mean?)
    let section2: &[u8; 3166] = &[0x00; 3166];
    raw.extend_from_slice(section2);

    // 0x1280: Volume name in AKAI format. (offset 4736)
    let section3: &[u8; 12] = &[
        0x18, 0x19, 0x1E, 0x0A, 0x18, 0x0B, 0x17, 0x0F, 0x0E, 0x0A, 0x0A, 0x0A
    ];
    raw.extend_from_slice(section3);

    let section4: Vec<u8> = match model {
        DeviceModel::S900 => [].into(),
        DeviceModel::S2000 => {
            let mut data = [0; 372];
            const DP4_S2000: [u8; 14] = [
                0x00, 0x00, 0x00, 0x11, 0x00, 0x01, 0x01,
                0x23, 0x00, 0x00, 0x32, 0x09, 0x0C, 0xFF
            ];
            data[..DP4_S2000.len()].copy_from_slice(&DP4_S2000);
            data.into()
        },
        DeviceModel::S3000 => {
            let mut data = [0; 372];
            const DP4_S3000: [u8; 14] = [
                0x00, 0x00, 0x32, 0x10, 0x00, 0x01, 0x01,
                0x00, 0x00, 0x00, 0x32, 0x09, 0x0C, 0xFF
            ];
            data[..DP4_S3000.len()].copy_from_slice(&DP4_S3000);
            data.into()
        }
    };
    raw.extend_from_slice(section4.as_slice());

    // 512 possible directory entries. (offset 5120)
    let section5: &[u8] = &[0; 12288];
    raw.extend_from_slice(section5);

    // 1583 * 1024 byte sectors for data. (offset 17408?)
    let section6: &[u8] = &[0; 1620992];
    raw.extend_from_slice(section6);

    raw
}

impl DeviceDisk<{ DeviceModel::S900 }> for Device<{ DeviceModel::S900 }>  {
    fn disk_size () -> usize {
        819200
    }
    fn fat_start () -> usize {
        1536
    }
    fn fat_end () -> usize {
        3136
    }
    fn header_start () -> usize {
        0
    }
    fn max_entries () -> usize {
        64
    }
    fn max_blocks () -> usize {
        796
    }
}

impl DeviceDisk<{ DeviceModel::S2000 }> for Device<{ DeviceModel::S2000 }> {
    fn disk_size () -> usize {
        1638400
    }
    fn fat_start () -> usize {
        1570
    }
    fn fat_end () -> usize {
        3166
    }
    fn header_start () -> usize {
        5120
    }
    fn max_entries () -> usize {
        512
    }
    fn max_blocks () -> usize {
        1583
    }
}

impl DeviceDisk<{ DeviceModel::S3000 }> for Device<{ DeviceModel::S3000 }> {
    fn disk_size () -> usize {
        1638400
    }
    fn fat_start () -> usize {
        1570
    }
    fn fat_end () -> usize {
        3166
    }
    fn header_start () -> usize {
        5120
    }
    fn max_entries () -> usize {
        512
    }
    fn max_blocks () -> usize {
        1583
    }
}

pub fn guess_model (volname: &[u8; 24]) -> DeviceModel {
    match volname[23] {
        0x00 => DeviceModel::S900,
        0x17 => DeviceModel::S2000,
        0x16 => DeviceModel::S3000,
        _ => panic!("could not determine device model from image data")
    }
}

impl DiskSamples for DiskImage<{ DeviceModel::S900 }> {
    fn header_length () -> usize {
        0x3c
    }
}

impl DiskSamples for DiskImage<{ DeviceModel::S2000 }> {
    fn header_length () -> usize {
        0xbe
    }
}

impl DiskSamples for DiskImage<{ DeviceModel::S3000 }> {
    fn header_length () -> usize {
        0xbe
    }
}
