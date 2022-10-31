use dawless_akai::*;

use std::io::{Read, Write};
use tabwriter::TabWriter;

fn main () {
    let args = std::env::args().collect::<Vec<String>>();

    akai_s3000()
        .load_disk(read(&args[1]))
        .list_files();

    let data = akai_s3000()
        //.load_disk(read(&args[1]))
        .blank_disk()
        .list_files()
        .add_sample("140VEC3BB1", &read(&args[2]))
        .list_files()
        .write_disk();

    //let disk = akai_s3000()
        //.blank_disk()
        //.add_sample("140VEC3BB11", &read(&args[1]));
}

fn print_files <'a> (files: impl Iterator<Item = &'a Sample<'a>>) {
    let mut tw = TabWriter::new(vec![]);
    write!(&mut tw, "NAME\tBYTES\tRATE\tLOOP\tSEMI\tCENT\tLENGTH\t").unwrap();
    for file in files {
        let Sample {
            name, size, sample_rate, loop_mode, tuning_semi, tuning_cent, length, ..
        } = file;
        write!(&mut tw, "\n{name}\t{size}\t{sample_rate:?}\t{loop_mode:?}\t{tuning_semi}\t{tuning_cent}\t{length}").unwrap();
    }
    tw.flush().unwrap();
    let output = String::from_utf8(tw.into_inner().unwrap()).unwrap();
    println!("\n{output}");
}

fn read (filename: &str) -> Vec<u8> {
    let mut f      = std::fs::File::open(&filename).expect("file not found");
    let metadata   = std::fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");
    buffer
}
