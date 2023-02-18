#![feature(let_chains)]

opt_mod::optional_module_flat!("cli": cli);

pub mod electribe2;
pub mod triton;

pub(crate) use thatsit::*;

pub(crate) fn read (filename: &std::path::Path) -> Vec<u8> {
    use std::io::Read;
    let mut f      = std::fs::File::open(&filename).expect("file not found");
    let metadata   = std::fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");
    buffer
}
