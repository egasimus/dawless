#[derive(Debug)]
pub struct Program {
    id:                   u8,       // 00      C       1       program header id
    addr_kg1:             u16,      // 01-02   x2      n/a     1st keygroup address (internal)
    name:                 String,   // 03-0e   a12             program name
    midi_program:         u8,       // 0f      C       0       MIDI program number (0-127)
    midi_channel:         u8,       // 10      C       0       MIDI channel (0-15, ff=omni)
    polyphony:            u8,       // 11      C       31      polyphony (1-32; 1-16 in S1000)
    priority:             u8,       // 12      C       1       priority (0=low, 1=normal, 2=high, 3=hold)
    range_low:            u8,       // 13      C       24      play range low (24-127 = C0-G8)
    range_hight:          u8,       // 14      C       127     play range high (24-127 = C0-G8)
    octave:               u8,       // 15      C       0       play octave (keyboard) shift (+/-2)
    output:               u8,       // 16      C       255     indivisual output (0-7, ff=off)
    volume:               u8,       // 17      C       99      stereo level
    pan:                  u8,       // 18      C       0       stereo pan
    loudness:             u8,       // 19      C       80      loudness
    vel_to_loudness:      u8,       // 1a      C       20      velocity > loud
    key_to_ loudness:     u8,       // 1b      C       0       key > loud
    pressure_to_loudness: u8,       // 1c      C       0       pressure > loud
    pan_lfo_rate:         u8,       // 1d      C       0       pan LFO rate
    pan_depth:            u8,       // 1e      C       99      pan depth
    pan_lfo_delay:        u8,       // 1f      C       0       pan LFO delay
    key_to_pan:           u8,       // 20      C       0       key > pan position 
    lfo_speed:            u8,       // 21      C       50      LFO speed
    lfo_depth:            u8,       // 22      C       0       LFO fixed depth
    lfo_delay:            u8,       // 23      C       0       LFO delay
    mod_to_depth:         u8,       // 24      C       30      modwheel > depth
    pressure_to_depth:    u8,       // 25      C       0       pressure > depth
    velocity_to_depth:    u8,       // 26      C       0       velocity > depth
    bend_to_pitch:        u8,       // 27      C       2       bendwheel > pitch
    pressure_to_pitch:    u8,       // 28      C       0       pressure > pitch
    keygroup_crossfade:   u8,       // 29      C       0       keygroup crossfade (0=off, 1=on)
    number_of_keygroups:  u8,       // 2a      C               # of keygroups (1-99)
    temp_program_number:  u8,       // 2b      C       n/a     temporary program number (internal)
    temperament:          [u8; 12], // 2c-37   C12             key temperament
    echo:                 u8,       // 38      C       0       echo output level (0=off, 1=on)
    modwheel_pan_amount:  u8,       // 39      C       0       modwheel pan amount
    retrigger:            u8,       // 3a      C       0       sample start coherence (0=off, 1=on)
    lfo_desync:           u8,       // 3b      C       0       LFO de-sync (0=off, 1=on) (def. 0)
    pitch_law:            u8,       // 3c      C       0       pitch law
    voice_assign_algo:    u8,       // 3d      C       0       voice assign algorithm (0=oldest, 1=quietest)
    pedal_to_loudness:    u8,       // 3e      C       10      soft pedal loudness reduction 
    pedal_to_attack:      u8,       // 3f      C       10      soft pedal attack stretch
    pedal_to_filter:      u8,       // 40      C       10      soft pedal filter close
    tune_offset:          u16,      // 41-42   v       0       tune offset
    key_to_lfo_rate:      u8,       // 43      C       0       key > LFO rate
    key_to_lfo_depth:     u8,       // 44      C       0       key > LFO depth
    key_to_lfo_delay:     u8,       // 45      C       0       key > LFO delay
    voice_output_scale:   u8,       // 46      C       50      voice output scale 
    stereo_output_scale:  u8,       // 47      C       0       stereo output scale
    keygroup: Vec<Keygroup>
}

#[derive(Debug)]
pub struct Keygroup {
    // 0000-0021       keygroup common data
    // 0022-
    // 
    // 00      C       2       keygroup block id
    // 01-02   v       n/a     next keygroup block address (internal)
    // 03      C       24      keyrange low
    // 04      C       127     keyrange high
    // 05-06   v       0       tune offset
    // 07      C       99      filter freq.
    // 08      C       0       key > filter freq.
    // 09      C       0       velocity > filter freq.
    // 0a      C       0       pressure > filter freq.
    // 0b      C       0       envelope > filter freq.
    // 0c      C       25      amp. attack
    // 0d      C       50      amp. decay
    // 0e      C       99      amp. sustain
    // 0f      C       45      amp. release
    // 10      C       0       velocity > amp. attack
    // 11      C       0       velocity > amp. release
    // 12      C       0       off velocity > amp. release
    // 13      C       0       key > decay & release 
    // 14      C       0       filter attack
    // 15      C       50      filter decay 
    // 16      C       99      filter sustain
    // 17      C       45      filter release
    // 18      C       0       velocity > filter attack
    // 19      C       0       velocity > filter relase
    // 1a      C       0       off velocity > fiter release
    // 1b      C       0       key > decay & release
    // 1c      C       25      velocity > filter envelope output
    // 1d      C       0       envelope > pitch 
    // 1e      C       1       velocity zone crossfade (0=off, 1=on)
    // 1f      C       n/a     # of velocity zones (internal)
    // 20      C       n/a     internal
    // 21      C       n/a     internal
    // 
    // 
    // 83      C       0       fixed rate detune
    // 84      C       0       attack hold until loop
    // 85-88   C4      0       constant pitch for zone 1--4 (0=track, 1=const); 84??
    // 89-8c   C4      0       output number offset for zone 1--4
    // 8d-94   v4      0       velocity > sample start
    // 95      C       0       velocity > loudness offset
    // 96-bf                   ??
    // 97      C       0       vel.  > filter freq.
    // 98      C       0       pres. > filter freq.
    // 99      C       0       env.  > filter freq.
    zones: [Zone; 4]
}

pub struct Zone {
    // 22-2d   A12             sample name
    // 2e      C       0       velocity range low
    // 2f      C       127     velocity range high
    // 30-31   v       0       tune offset
    // 32      C       0       loudness offset
    // 33      C       0       filter freq. offset
    // 34      c       0       pan offset
    // 35      C       0       loop in relase
    // 36      C       n/a     low velocity xfade factor (intarnal)
    // 37      C       n/a     low velocity xfade factor (intarnal)
    // 38-39   v       n/a     sample header block address (intarnal)
    // 
    // 3a-52                   velocity zone 2
    // 53-69                   velocity zone 3
    // 6a-82                   velocity zone 4
}
