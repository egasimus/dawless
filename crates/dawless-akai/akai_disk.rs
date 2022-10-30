use crate::*;

pub struct DiskImage<const M: DeviceModel, const N: usize> {
    pub data: Vec<u8>
}

pub trait DeviceDisk<const M: DeviceModel, const N: usize> {
    fn disk (&self) -> DiskImage<M, N>;

    fn load (&self, raw: Vec<u8>) -> DiskFiles<M, N> {
        DiskFiles::load(raw.as_slice())
    }
}

impl DeviceDisk<{ DeviceModel::S900 }, 819200> for Device<{ DeviceModel::S900 }>  {
    fn disk (&self) -> DiskImage<{ DeviceModel::S900 }, 819200> {
        let mut image = DiskImage { data: Vec::with_capacity(819200) };
        image.blank();
        image
    }
}

impl DeviceDisk<{ DeviceModel::S2000 }, 1638400> for Device<{ DeviceModel::S2000 }> {
    fn disk (&self) -> DiskImage<{ DeviceModel::S2000 }, 1638400> {
        let mut image = DiskImage { data: Vec::with_capacity(1638400) };
        image.blank();
        image
    }
}

impl DeviceDisk<{ DeviceModel::S3000 }, 1638400> for Device<{ DeviceModel::S3000 }> {
    fn disk (&self) -> DiskImage<{ DeviceModel::S3000 }, 1638400> {
        let mut image = DiskImage { data: Vec::with_capacity(1638400) };
        image.blank();
        image
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

pub fn get_model_parameters (model: &DeviceModel) -> (usize, usize, usize, usize, usize) {
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
