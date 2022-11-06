/// Pattern start tag
const PTST: [u8; 4] = [80, 84, 83, 84];

/// Pattern end tag
const PTED: [u8; 4] = [80, 84, 69, 68];

const PATTERNS_OFFSET: usize = 0x10100;

const PATTERN_SIZE:    usize = 0x4000;

const PARTS_OFFSET:    usize = 0x0800;

const PART_SIZE:       usize = 0x0330;

const STEPS_OFFSET:    usize = 0x001e;

const STEP_SIZE:       usize = 0x000c;

#[derive(Debug)]
pub struct Electribe2AllPatterns {
    pub patterns: Vec<Electribe2Pattern>
}

impl Electribe2AllPatterns {
    /// Create an empty pattern bundle
    pub fn empty () -> Self {
        Self { patterns: Vec::with_capacity(250) }
    }
    /// Read a pattern bundle
    pub fn read (raw: &[u8]) -> Self {
        let mut patterns = vec![];
        for index in 0..250 {
            let start = PATTERNS_OFFSET + index * PATTERN_SIZE;
            let end   = start + PATTERN_SIZE;
            let pattern = Electribe2Pattern::read(&raw[start..end]);
            patterns.push(pattern);
        }
        Self { patterns }
    }
}

#[derive(Debug, Default, Clone)]
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

impl Electribe2Pattern {
    pub fn read (raw: &[u8]) -> Self {
        assert_eq!(&raw[0x0000..0x0004], &PTST);
        let mut pattern = Self::default();
        pattern.name = String::from_utf8(raw[0x0010..0x0020].into())
            .expect("invalid pattern name");
        for i in 0..16 {
            let start = PARTS_OFFSET + i * 0x330;
            let end   = start + PART_SIZE;
            pattern.parts.push(Electribe2Part::read(&raw[start..end]))
        }
        assert_eq!(&raw[0x3BFC..0x3C00], &PTED);
        pattern
    }
}

#[derive(Debug, Default, Clone)]
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
    pub sample:           u16,
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
    /// 0x0014 - envelope attach
    pub attack:           u8,
    /// 0x0015 - envelope decay
    pub decay:            u8,
    /// 0x0018 - volume
    pub level:            u8,
    /// 0x0019 - stereo pan
    pub pan:              u8,
    /// 0x001a - envelope controls amp or just filter?
    pub amp_eg:           u8,
    /// 0x001b - route to master effect?
    pub mfx_on:           u8,
    /// 0x001c - groove type
    pub groove_type:      u8,
    /// 0x001d - amount of groove to apply
    pub groove_depth:     u8,
    /// 0x0020 - insert effect toggle
    pub ifx_on:           u8,
    /// 0x0021 - insert effect type
    pub ifx_type:         u8,
    /// 0x0022 - insert effect parameter
    pub ifx_param:        u8,
    /// 0x0024 - oscillator tuning
    pub pitch:            u8,
    /// 0x0025 - portamento
    pub glide:            u8,
    /// 64 steps of 12 bytes each
    pub steps:            Vec<Electribe2Step>
}

impl Electribe2Part {
    pub fn read (raw: &[u8]) -> Self {
        let mut part = Self::default();
        part.last_step        = raw[0x00];
        part.voice_assign     = raw[0x02];
        part.priority         = raw[0x03];
        part.motion_seq       = raw[0x04];
        part.trigger_vel      = raw[0x05];
        part.scale            = raw[0x06];
        part.sample           = (raw[0x08] as u16) + (0x100u16 * raw[0x09] as u16);
        part.osc              = raw[0x0b];
        part.filter_type      = raw[0x0c];
        part.filter_cutoff    = raw[0x0d];
        part.filter_resonance = raw[0x0e];
        part.filter_envelope  = raw[0x0f];
        part.modulation_type  = raw[0x10];
        part.modulation_speed = raw[0x11];
        part.modulation_depth = raw[0x12];
        part.attack           = raw[0x14];
        part.decay            = raw[0x15];
        part.level            = raw[0x18];
        part.pan              = raw[0x19];
        part.amp_eg           = raw[0x1a];
        part.mfx_on           = raw[0x1b];
        part.groove_type      = raw[0x1c];
        part.groove_depth     = raw[0x1d];
        part.ifx_on           = raw[0x20];
        part.ifx_type         = raw[0x21];
        part.ifx_param        = raw[0x22];
        part.pitch            = raw[0x23];
        part.glide            = raw[0x24];
        for index in 0..64 {
            let start = STEPS_OFFSET + index * STEP_SIZE;
            let end   = start + STEP_SIZE;
            part.steps.push(Electribe2Step::read(&raw[start..end]));
        }
        part
    }
}

#[derive(Debug, Default, Clone)]
pub struct Electribe2Step {
    /// 0x00
    pub empty:    u8,
    /// 0x01
    pub gate:     u8,
    /// 0x02
    pub velocity: u8,
    /// 0x03
    pub chord:    u8,
    /// 0x04
    pub note_1:   u8,
    /// 0x05
    pub note_2:   u8,
    /// 0x06
    pub note_3:   u8,
    /// 0x07
    pub note_4:   u8
}

impl Electribe2Step {
    pub fn read (raw: &[u8]) -> Self {
        let mut step = Self::default();
        step.empty    = raw[0x00];
        step.gate     = raw[0x01];
        step.velocity = raw[0x02];
        step.chord    = raw[0x03];
        step.note_1   = raw[0x04];
        step.note_2   = raw[0x05];
        step.note_3   = raw[0x06];
        step.note_4   = raw[0x07];
        step
    }
}

#[cfg(feature = "cli")] pub use cli::*;
#[cfg(feature = "cli")] mod cli {
    use super::*;
    use std::io::{Read, Write};
    use std::path::PathBuf;

    #[derive(clap::Subcommand)]
    pub enum Electribe2 {

        /// Manage pattern files
        Patterns {
            /// Import an existing e2sSample.all pattern bundle.
            #[clap(long)]
            import: Option<PathBuf>,
            /// Add a pattern
            #[clap(long)]
            add:    Vec<PathBuf>,
            /// Pick a pattern by number and append it to a new pattern bundle.
            #[clap(long)]
            pick:   Option<Vec<usize>>,
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

    pub(crate) fn cli (command: &Electribe2) {

        match command {

            Electribe2::Patterns { import, add, pick } => {

                if let Some(import) = import {
                    let data = read(import);
                    let mut bundle = Electribe2AllPatterns::read(&data);
                    for (index, pattern) in bundle.patterns.iter().enumerate() {
                        println!("{:>3} {}", index+1, pattern.name);
                    }

                    if let Some(pick) = pick {
                        let mut new_bundle = Electribe2AllPatterns::empty();
                        for index in pick {
                            new_bundle.patterns.push(
                                bundle.patterns.get(*index-1).unwrap().clone()
                            );
                        }
                        println!("");
                        for (index, pattern) in new_bundle.patterns.iter().enumerate() {
                            println!("{:>3} {}", index+1, pattern.name);
                        }
                    }
                }

            },

            Electribe2::Samples { import, add } => {
            }

        }

    }

    fn read (filename: &std::path::Path) -> Vec<u8> {
        let mut f      = std::fs::File::open(&filename).expect("file not found");
        let metadata   = std::fs::metadata(&filename).expect("unable to read metadata");
        let mut buffer = vec![0; metadata.len() as usize];
        f.read(&mut buffer).expect("buffer overflow");
        buffer
    }

    impl std::fmt::Display for Electribe2Pattern {
        fn fmt (&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{:<30}", self.name)?;
            for part in self.parts.iter() {
                write!(f, "\n  {}", part)?;
            }
            Ok(())
        }
    }

    impl std::fmt::Display for Electribe2Part {
        fn fmt (&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{}", self.sample);
            for step in self.steps.iter() {
                write!(f, "\n    {}", step)?;
            }
            Ok(())
        }
    }

    impl std::fmt::Display for Electribe2Step {
        fn fmt (&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "[{} {} {} {} {} {} {} {}]",
                self.empty,
                self.gate,
                self.velocity,
                self.chord,
                self.note_1,
                self.note_2,
                self.note_3,
                self.note_4
            )
        }
    }
}
