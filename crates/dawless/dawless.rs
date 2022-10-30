use dawless_akai::*;

use std::io::Read;

fn main () {
    //let bytes = read("../s2kdie.1.1.3/file");
    //let bytes = read("../s2kdie.1.1.3/file2");
    //let bytes = read("../s2kdie.1.1.3/file3");
    let files = akai_s3000().load(read("../s2kdie.1.1.3/file3")).list();
    println!("{files:?}");
}

fn read (filename: &str) -> Vec<u8> {
    let mut f      = std::fs::File::open(&filename).expect("file not found");
    let metadata   = std::fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");
    buffer
}
