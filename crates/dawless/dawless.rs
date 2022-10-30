use dawless_akai::*;

use std::io::{Read, Write};
use tabwriter::TabWriter;

fn main () {
    //let bytes = read("../s2kdie.1.1.3/file");
    //let bytes = read("../s2kdie.1.1.3/file2");
    //let bytes = read("../s2kdie.1.1.3/file3");
    let files = akai_s3000().load(read("../s2kdie.1.1.3/file3")).list();
    if files.len() > 0 {
        let mut tw = TabWriter::new(vec![]);
        write!(&mut tw, "NAME\tRATE\tLOOP\tSEMI\tCENT\tLENGTH\t");
        for file in files {
            let Sample { name, sample_rate, loop_mode, tuning_semi, tuning_cent, length } = file;
            write!(&mut tw, "\n{name}\t{sample_rate:?}\t{loop_mode:?}\t{tuning_semi}\t{tuning_cent}\t{length}");
        }
        tw.flush().unwrap();
        let output = String::from_utf8(tw.into_inner().unwrap()).unwrap();
        println!("\n{output}");
    }
}

fn read (filename: &str) -> Vec<u8> {
    let mut f      = std::fs::File::open(&filename).expect("file not found");
    let metadata   = std::fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");
    buffer
}
