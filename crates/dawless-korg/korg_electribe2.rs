#[cfg(feature = "cli")]
use std::io::{Read, Write};
use std::path::PathBuf;

#[cfg(feature = "cli")]
#[derive(clap::Subcommand)]
pub enum Electribe2 {

    /// Manage pattern files
    Patterns {
        /// Import an existing e2sSample.all pattern bundle.
        #[clap(long)]
        import: Option<PathBuf>,
        /// Add a pattern
        #[clap(long)]
        add: Vec<PathBuf>,
    },

    /// Manage sample files
    Samples {
        /// Import an existing e2sSample.all sample bundle.
        #[clap(long)]
        import: Option<String>,
        /// Add a sample
        #[clap(long)]
        add: Vec<PathBuf>,
    }

}

#[cfg(feature = "cli")]
pub(crate) fn cli (command: &Electribe2) {

    match command {

        Electribe2::Patterns { import, add } => {
            if let Some(import) = import {
                let data = read(import);
                let patterns = Electribe2AllPatterns::read(&data);
                println!("{patterns:?}");
            }
        },

        Electribe2::Samples { import, add } => {
        }

    }

}

#[cfg(feature = "cli")]
fn read (filename: &std::path::Path) -> Vec<u8> {
    let mut f      = std::fs::File::open(&filename).expect("file not found");
    let metadata   = std::fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");
    buffer
}

#[derive(Debug)]
pub struct Electribe2AllPatterns {
    pub patterns: Vec<Electribe2Pattern>
}

impl Electribe2AllPatterns {
    pub fn read (raw: &[u8]) -> Self {
        let mut patterns = vec![];
        let size = 0x4000;
        for index in 0..250 {
            let address = 0x10100 + index * size;
            let pattern = Electribe2Pattern::read(&raw[address..address+size]);
            patterns.push(pattern);
        }
        Self { patterns }
    }
}

#[derive(Debug, Default)]
pub struct Electribe2Pattern {
    /// 0x0010..0x0020 - name
    pub name:      String,
    /// 0x0012..0x0013 - bpm
    pub bpm:       u16,
    /// 0x0014 - swing
    pub swing:     u8,
    /// 0x0015 - length
    pub length:    u8,
    /// 0x0016 - beats
    pub beats:     u8,
    /// 0x0017 - key
    pub key:       u8,
    /// 0x0018 - scale
    pub scale:     u8,
    /// 0x0019 - chord set
    pub chord_set: u8,
    /// 0x001a - level
    pub level:     u8,
    /// 0x0021 - gate arp
    pub gate_arp:  u8,
    /// 0x002d - master fx type
    pub mfx_type:  u8,
    /// 0x0034 - alt 13/14
    pub alt_13_14: u8,
    /// 0x0035 - alt 15/16
    pub alt_15_16: u8,
    /// 0x0800..0x0b30 - one track (816 bytes)
    pub parts:    Vec<Electribe2Part>
}

/// Pattern start tag
const PTST: [u8; 4] = [80, 84, 83, 84];

/// Pattern end tag
const PTED: [u8; 4] = [80, 84, 69, 68];

impl Electribe2Pattern {
    pub fn read (raw: &[u8]) -> Self {
        assert_eq!(&raw[0x0000..0x0004], &PTST);
        let name = String::from_utf8(raw[0x0010..0x0020].into())
            .expect("invalid pattern name");
        let mut parts = Vec::with_capacity(16);
        for i in 0..16 {
            let start = 0x800 + i * 0x330;
            let end = start + 0x330;
            parts.push(Electribe2Part::read(&raw[start..end]))
        }
        assert_eq!(&raw[0x3BFC..0x3C00], &PTED);
        Self { name, parts, ..Default::default() }
    }
}

#[derive(Debug, Default)]
pub struct Electribe2Part {
    /// 0x0000 - last step
    pub last_step:        u8,
    /// 0x0002 - voice assign
    pub voice_assign:     u8,
    /// 0x0003 - part priority
    pub priority:         u8,
    /// 0x0004 - motion sequence enable
    pub motion_seq:       u8,
    /// 0x0005 - velocity curve
    pub trigger_vel:      u8,
    /// 0x0006 - scale mode
    pub scale:            u8,
    /// 0x0008..0x0009 - oscilator/sample number
    pub sample:           u8,
    /// 0x000b - osc edit
    pub osc:              u8,
    /// 0x000c - filter type
    pub filter_type:      u8,
    /// 0x000d - filter cutoff
    pub filter_cutoff:    u8,
    /// 0x000e - filter resonance
    pub filter_resonance: u8,
    /// 0x000f - filter envelope
    pub filter_envelope:  u8,
    /// 0x0010 - modulation type
    pub modulation_type:  u8,
    /// 0x0011 - modulation speed
    pub modulation_speed: u8,
    /// 0x0012 - modulation depth
    pub modulation_depth: u8,
    /// 0x0014
    pub attack:           u8,
    /// 0x0015
    pub decay:            u8,
    /// 0x0018
    pub level:            u8,
    /// 0x0019
    pub pan:              u8,
    /// 0x001a
    pub amp_eg:           u8,
    /// 0x001b
    pub mfx:              u8,
    /// 0x001c
    pub groove_type:      u8,
    /// 0x001d
    pub groove_depth:     u8,
    /// 0x0020
    pub ifx_type:         u8,
    /// 0x0021
    pub mfx_on:           u8,
    /// 0x0022
    pub ifx_amount:       u8,
    /// 0x0024
    pub pitch:            u8,
    /// 0x0025
    pub glide:            u8,
    /// 64 steps of 12 bytes each
    pub steps:            Vec<Electribe2Step>
}

impl Electribe2Part {
    pub fn read (raw: &[u8]) -> Self {
        Default::default()
    }
}

#[derive(Debug, Default)]
pub struct Electribe2Step {
    /// 0x0000
    pub empty:    u8,
    /// 0x0001
    pub gate:     u8,
    /// 0x0002
    pub velocity: u8,
    /// 0x0003
    pub chord:    u8,
    /// 0x0004
    pub note_1:   u8,
    /// 0x0005
    pub note_2:   u8,
    /// 0x0006
    pub note_3:   u8,
    /// 0x0008
    pub note_4:   u8
}
