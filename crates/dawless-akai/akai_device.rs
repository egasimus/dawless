use crate::*;

pub struct Device<const M: DeviceModel>;
#[derive(PartialEq, Eq)] pub enum DeviceModel { S900, S2000, S3000 }
pub fn akai_s900  () -> Device<{ DeviceModel::S900 }>  { Device }
pub fn akai_s2000 () -> Device<{ DeviceModel::S2000 }> { Device }
pub fn akai_s3000 () -> Device<{ DeviceModel::S3000 }> { Device }
impl DeviceDisk<{ DeviceModel::S900 }>  for Device<{ DeviceModel::S900 }>  {}
impl DeviceDisk<{ DeviceModel::S2000 }> for Device<{ DeviceModel::S2000 }> {}
impl DeviceDisk<{ DeviceModel::S3000 }> for Device<{ DeviceModel::S3000 }> {}

pub fn disk_capacity (model: &DeviceModel) -> usize {
    match model {
        DeviceModel::S900 => 0x0C8000, // 819200 bytes
        _                 => 0x190000, // 1638400 bytes
    }
}

pub fn file_headers_offset (model: &DeviceModel) -> usize {
    match model {
        DeviceModel::S900 => 0x0000, // from byte 0
        _                 => 0x1400, // from byte 5120
    }
}

pub fn max_files (model: &DeviceModel) -> usize {
    match model {
        DeviceModel::S900 => 0x40,  // 64 entries
        _                 => 0x200, // 512 entries
    }
}

pub fn file_table_boundaries (model: &DeviceModel) -> (usize, usize) {
    match model {
        DeviceModel::S900 => (0x0600, 0x0c40), // from byte 1536 to 3136
        _                 => (0x0622, 0x0c5e), // from byte 1570 to 3166
    }
}

pub fn max_blocks (model: &DeviceModel) -> usize {
    match model {
        DeviceModel::S900 => 0x031c, // 796 blocks
        _                 => 0x062f, // 1583 blocks
    }
}

pub fn guess_model (volname: &[u8; 24]) -> DeviceModel {
    match volname[23] {
        0x00 => DeviceModel::S900,
        0x17 => DeviceModel::S2000,
        0x16 => DeviceModel::S3000,
        _    => panic!("could not determine device model from image data")
    }
}
