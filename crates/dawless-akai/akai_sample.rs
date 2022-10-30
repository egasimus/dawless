#[derive(Debug)]
pub struct Sample {
    pub name:        String,
    pub sample_rate: SampleRate,
    pub loop_mode:   LoopMode,
    pub tuning_semi: u8,
    pub tuning_cent: u8,
    pub length:      u32
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

