use crate::*;

pub trait DiskLoad<const M: DeviceModel, const N: usize, const O: usize>: Sized {
    fn load (&self, bytes: &[u8]) -> DiskFiles<M, O> {
        if bytes.len() < N {
            panic!("disk image should be at least {N} bytes")
        }
        DiskFiles::load(bytes)
    }
}

impl DiskLoad<{ DeviceModel::S900 }, 819200, 64> for DiskImage<{ DeviceModel::S900 }, 819200> {}
impl DiskLoad<{ DeviceModel::S2000 }, 1638400, 512> for DiskImage<{ DeviceModel::S3000 }, 1638400> {}
impl DiskLoad<{ DeviceModel::S3000 }, 1638400, 512> for DiskImage<{ DeviceModel::S3000 }, 1638400> {}
