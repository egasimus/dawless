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
