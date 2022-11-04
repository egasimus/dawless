use std::io::{Read, Write};
use std::fs::File;
use std::path::PathBuf;
use std::ops::Deref;
use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    brand: Brand,
}

#[derive(Subcommand)]
enum Brand {
    /// Tools for AKAI devices
    AKAI {
        #[command(subcommand)]
        model: dawless_akai::AKAI,
    },
    /// Tools for Korg devices
    Korg {
        #[command(subcommand)]
        model: dawless_korg::Korg,
    },
}

fn main () {
    let cli = Cli::parse();

    match &cli.brand {
        Brand::AKAI { model } => dawless_akai::cli(model),
        Brand::Korg { model } => dawless_korg::cli(model),
    };

    //let args = std::env::args().collect::<Vec<String>>();

    ////akai_s3000()
        ////.load_disk(&read(&args[1]))
        ////.list_files();

    //let data = akai_s3000()
        ////.load_disk(read(&args[1]))
        //.blank_disk()
        //.list_files()
        ////.add_sample("140VEC3BB1", &read(&args[2]))
        ////.add_sample("140VEC3BB2", &read(&args[2]))
        //.add_sample("CL-VEC3-031",   &read("../KIT/CL-VEC3-031.wav"))
        //.add_sample("RD-VEC3-021",   &read("../KIT/RD-VEC3-021.wav"))
        //.add_sample("OH-VEC3-048",   &read("../KIT/OH-VEC3-048.wav"))
        //.add_sample("CH-VEC3-54",    &read("../KIT/CH-VEC3-54.wav"))
        //.add_sample("CH-VEC3-40",    &read("../KIT/CH-VEC3-40.wav"))
        //.add_sample("CC-VEC321",     &read("../KIT/CC-VEC321.wav"))
        //.add_sample("BS-C-VEC3-002", &read("../KIT/BS-C-VEC3-002.wav"))
        //.add_sample("BD-VEC3-T77",   &read("../KIT/BD-VEC3-T77.wav"))
        //.add_sample("BD-VEC3-T47",   &read("../KIT/BD-VEC3-T47.wav"))
        //.add_sample("BD-VEC3-T07",   &read("../KIT/BD-VEC3-T07.wav"))
        //.add_sample("BD-VEC3-D08",   &read("../KIT/BD-VEC3-D08.wav"))
        //.add_sample("BD-VEC3-D07",   &read("../KIT/BD-VEC3-D07.wav"))
        //.add_sample("BD-VEC3-D01",   &read("../KIT/BD-VEC3-D01.wav"))
        //.list_files()
        //.write_disk();

    //akai_s3000()
        //.load_disk(&data)
        //.list_files();

    //let mut f = std::fs::File::create("data/TEST.IMG").unwrap();
    //f.write_all(data.as_slice()).unwrap();
    //println!("Wrote data/TEST.img");

    //let disk = akai_s3000()
        //.blank_disk()
        //.add_sample("140VEC3BB11", &read(&args[1]));
}

//use tabwriter::TabWriter;
//fn print_files <'a> (files: impl Iterator<Item = &'a Sample<'a>>) {
    //let mut tw = TabWriter::new(vec![]);
    //write!(&mut tw, "NAME\tBYTES\tRATE\tLOOP\tSEMI\tCENT\tLENGTH\t").unwrap();
    //for file in files {
        //let Sample {
            //name, size, sample_rate, loop_mode, tuning_semi, tuning_cent, length, ..
        //} = file;
        //write!(&mut tw, "\n{name}\t{size}\t{sample_rate:?}\t{loop_mode:?}\t{tuning_semi}\t{tuning_cent}\t{length}").unwrap();
    //}
    //tw.flush().unwrap();
    //let output = String::from_utf8(tw.into_inner().unwrap()).unwrap();
    //println!("\n{output}");
//}

fn read (filename: &std::path::Path) -> Vec<u8> {
    let mut f      = std::fs::File::open(&filename).expect("file not found");
    let metadata   = std::fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");
    buffer
}
