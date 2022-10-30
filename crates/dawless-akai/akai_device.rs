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

impl DiskBlank<819200> for DiskImage<{ DeviceModel::S900 }, 819200> {
    fn data_mut (&mut self) -> &mut Vec<u8> {
        &mut self.data
    }
    fn section1 (&self) -> [u8; 24] {
        [
            0x0A, 0x0A, 0x0A, 0x0A, 0x0A, 0x0A, 0x0A, 0x0A,
            0x0A, 0x0A, 0x0A, 0x0A, 0x00, 0x00, 0x06, 0x0A,
            0xFF, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00
        ]
    }
    fn section4 (&self) -> [u8; 372] {
        unreachable!("S900 images have no dp4 section")
    }
}

impl DiskBlank<1638400> for DiskImage<{ DeviceModel::S2000 }, 1638400> {
    fn data_mut (&mut self) -> &mut Vec<u8> {
        &mut self.data
    }
    fn section1 (&self) -> [u8; 24] {
        [
            0x0A, 0x0A, 0x0A, 0x0A, 0x0A, 0x0A, 0x0A, 0x0A,
            0x0A, 0x0A, 0x0A, 0x0A, 0x00, 0x00, 0x06, 0x0A,
            0xFF, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x17
        ]
    }
    fn section4 (&self) -> [u8; 372] {
        let mut data = [0; 372];
        const DP4_S2000: [u8; 14] = [
            0x00, 0x00, 0x00, 0x11, 0x00, 0x01, 0x01,
            0x23, 0x00, 0x00, 0x32, 0x09, 0x0C, 0xFF
        ];
        data[..DP4_S2000.len()].copy_from_slice(&DP4_S2000);
        data
    }
}

impl DiskBlank<1638400> for DiskImage<{ DeviceModel::S3000 }, 1638400> {
    fn data_mut (&mut self) -> &mut Vec<u8> {
        &mut self.data
    }
    fn section1 (&self) -> [u8; 24] {
        [
            0x0A, 0x0A, 0x0A, 0x0A, 0x0A, 0x0A, 0x0A, 0x0A,
            0x0A, 0x0A, 0x0A, 0x0A, 0x00, 0x00, 0x06, 0x0A,
            0xFF, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x16
        ]
    }
    fn section4 (&self) -> [u8; 372] {
        let mut data = [0; 372];
        const DP4_S3000: [u8; 14] = [
            0x00, 0x00, 0x32, 0x10, 0x00, 0x01, 0x01,
            0x00, 0x00, 0x00, 0x32, 0x09, 0x0C, 0xFF
        ];
        data[..DP4_S3000.len()].copy_from_slice(&DP4_S3000);
        data
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

pub fn guess_model (volname: &[u8; 24]) -> DeviceModel {
    match volname[23] {
        0x00 => DeviceModel::S900,
        0x17 => DeviceModel::S2000,
        0x16 => DeviceModel::S3000,
        _ => panic!("could not determine device model from image data")
    }
}
