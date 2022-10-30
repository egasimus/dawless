use crate::*;

pub struct Device<const M: DeviceModel>;

#[derive(PartialEq, Eq)]
pub enum DeviceModel {
    S900,
    S1000,
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

impl DeviceDisk<{ DeviceModel::S900 }> for Device<{ DeviceModel::S900 }>  {
    fn disk_size () -> usize {
        819200
    }
    fn section1 (&self) -> [u8; 24] {
        [
            0x0A, 0x0A, 0x0A, 0x0A, 0x0A, 0x0A, 0x0A, 0x0A,
            0x0A, 0x0A, 0x0A, 0x0A, 0x00, 0x00, 0x06, 0x0A,
            0xFF, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00
        ]
    }
    fn section4 (&self) -> [u8; 372] {
        unreachable!("S900 images have no section4")
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
    fn write_header (&mut self, header: [u8; 24]) -> &mut Self {
        //self.head[0] = header;
        self
    }
    fn write_size (&mut self, size: u32) -> &mut Self {
        //self.size[0] = size;
        self
    }
    fn write_data (&mut self, data: &[u8]) -> &mut Self {
        //self.data[0] = data;
        self
    }
}

impl DiskSamples for DiskImage<{ DeviceModel::S2000 }> {
    fn header_length () -> usize {
        0xbe
    }
    fn write_header (&mut self, header: [u8; 24]) -> &mut Self {
        //self.head[0] = header;
        self
    }
    fn write_size (&mut self, size: u32) -> &mut Self {
        //self.size[0] = size;
        self
    }
    fn write_data (&mut self, data: &[u8]) -> &mut Self {
        //self.data[0] = data;
        self
    }
}

impl DiskSamples for DiskImage<{ DeviceModel::S3000 }> {
    fn header_length () -> usize {
        0xbe
    }
    fn write_header (&mut self, header: [u8; 24]) -> &mut Self {
        //self.head[0] = header;
        self
    }
    fn write_size (&mut self, size: u32) -> &mut Self {
        //self.size[0] = size;
        self
    }
    fn write_data (&mut self, data: &[u8]) -> &mut Self {
        //self.data[0] = data;
        self
    }
}
