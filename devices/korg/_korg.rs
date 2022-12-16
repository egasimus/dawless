opt_mod::optional_module_flat!("cli": cli @ "_korg-cli.rs");
opt_mod::module!(electribe2);
opt_mod::module!(triton);

pub(crate) fn read (filename: &std::path::Path) -> Vec<u8> {
    use std::io::Read;
    let mut f      = std::fs::File::open(&filename).expect("file not found");
    let metadata   = std::fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");
    buffer
}
