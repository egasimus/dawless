use crate::*;

pub trait DiskBlank<const N: usize>: Sized {
    fn data_mut (&mut self) -> &mut Vec<u8>;

    fn put_data <const L: usize> (&mut self, index: usize, section: &[u8; L]) -> usize {
        let length = section.len();
        self.data_mut()[index..index+length].copy_from_slice(section);
        index + length
    }

    fn blank (&mut self) {
        let index  = 0;
        let index  = self.put_data(index, &self.section1());
        let index  = self.put_data(index, &self.section2());
        let index  = self.put_data(index, &self.section3());
        let index  = self.put_data(index, &self.section4());
        let index  = self.put_data(index, &self.section5());
        let _index = self.put_data(index, &self.section6());
    }

    /** Disk header. */
    fn section1 (&self) -> [u8; 24];

    /** Bunch of empty space. (what does it mean?) */
    fn section2 (&self) -> [u8; 3166] {
        [0; 3166]
    }

    /** Volume name in AKAI format. (offset 4736) */
    fn section3 (&self) -> [u8; 12] {
        [0x18, 0x19, 0x1E, 0x0A, 0x18, 0x0B, 0x17, 0x0F, 0x0E, 0x0A, 0x0A, 0x0A]
    }

    /** Not supported on S900, different on S2000 vs S3000. (what does it mean?) */
    fn section4 (&self) -> [u8; 372];

    /** 512 possible directory entries. (offset 5120) */
    fn section5 (&self) -> [u8; 12288] {
        [0; 12288]
    }

    /** 1583 * 1024 byte sectors for data. (offset 17408?) */
    fn section6 (&self) -> [u8; 1620992] {
        [0; 1620992]
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
