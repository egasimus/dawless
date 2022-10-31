use crate::*;

pub fn sample_header_length (model: &DeviceModel) -> usize {
    match model {
        DeviceModel::S900 => 0x3c, // 796 blocks
        _                 => 0xbe, // 1583 blocks
    }
}

#[derive(Debug)]
pub struct Sample<'a> {
    pub name:        String,
    pub size:        u32,
    pub data:        &'a [u8],
    pub sample_rate: SampleRate,
    pub loop_mode:   LoopMode,
    pub tuning_semi: u8,
    pub tuning_cent: u8,
    pub length:      u32
}

impl<'a> Sample<'a> {
    pub fn serialize (name: &str, data: &[u8]) -> Vec<u8> {
        let header_length = 0xbe;
        let compression = data[20];
        if compression != 1 {
            panic!("uncompressed wavs only")
        }
        let bitrate = data[34];
        if bitrate != 16 {
            panic!("16-bit wavs only")
        }
        let sample_rate = u32::from_le_bytes([data[24], data[25], data[26], data[27]]);
        if sample_rate != 44100 {
            panic!("44.1kHz wavs only")
        }
        let channels = data[22];
        if channels != 1 {
            panic!("mono wavs only")
        }
        let contents = &data[44..];
        let length = (contents.len() as u32).to_le_bytes();
        let max = header_length + contents.len();
        let mut output = vec![0x00; max];
        output[0x00] = 0x03; // format (S3000)
        let channels = data[22];
        if channels != 1 {
            panic!("mono wavs only")
        }
        output[0x01] = channels; // channels
        output[0x02] = 0x3C;     // original pitch
        // name
        let name_akai = str_to_name(name);
        put_vec(&mut output, 0x03, &name_akai[..usize::min(name_akai.len(), 12)]);
        output[0x0f] = 0x80; // valid
        output[0x10] = 0x01; // loops
        output[0x11] = 0x00; // first loop
        output[0x12] = 0x00; // dummy
        output[0x13] = 0x02; // don't loop
        output[0x14] = 0x00; // tune cent
        output[0x15] = 0x00; // tune semi
        // data abs. start addr. (internal?)
        put_vec(&mut output, 0x16, &[0x00, 0x04, 0x01, 0x00]);
        // set sample length
        put_vec(&mut output, 0x1a, &length);
        // set sample start
        put_vec(&mut output, 0x1e, &[0x00, 0x00, 0x00, 0x00]);
        // set sample end
        put_vec(&mut output, 0x22, &length);
        // set sample rate
        put_vec(&mut output, 0x8a, &sample_rate.to_le_bytes());
        // copy sample data
        put_vec(&mut output, header_length, &contents);

        output
    }
}

#[derive(Debug)]
pub enum SampleRate {
    Hz22050,
    Hz44100
}

#[derive(Debug)]
pub enum LoopMode {
    Normal,
    UntilRelease,
    NoLoop,
    PlayToEnd
}

