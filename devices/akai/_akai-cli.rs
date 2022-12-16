use super::*;
use std::io::{Read, Write};
use std::fs::{File, metadata};
use std::path::{Path, PathBuf};

#[derive(clap::Subcommand)]
pub enum AKAI {
    /// Tools for the AKAI S3000
    S3000 {
        /// Import an existing disk image
        #[clap(long)]
        import:  Vec<std::path::PathBuf>,

        /// Add a sample to the disk image
        #[clap(long)]
        sample:  Vec<std::path::PathBuf>,

        /// Add a program to the disk image
        #[clap(long)]
        program: Vec<String>,

        /// Add a multi to the disk image
        #[clap(long)]
        multi:   Vec<String>,

        /// Set the disk label
        #[clap(long)]
        label:   Option<String>,

        /// Write the disk image to a file
        #[clap(long)]
        export:  Option<std::path::PathBuf>,
    }
}

pub fn run_cli (device: &AKAI) {
    match device {
        AKAI::S3000 { import, sample, export, .. } => {
            let mut disk = crate::s3kxl::akai_s3000().blank_disk();
            for path in import {
                println!("Importing {path:?}");
                // fixme: allow multiple disks to be imported into one
                disk = akai_s3000().load_disk(&read(&path));
                disk = disk.list_files();
            }
            for path in sample {
                if let Some(stem) = path.file_stem() {
                    let stem = stem.to_string_lossy();
                    let akai = str_to_name(&stem);
                    let name = u8_to_string(&akai);
                    println!("Importing {path:?} as {}", &name);
                    disk = disk.add_sample(&name, &read(path));
                    disk = disk.list_files();
                } else {
                    println!("Ignoring file.")
                }
            }
            if let Some(path) = export {
                std::fs::File::create(path)
                    .unwrap()
                    .write_all(disk.write_disk().as_slice())
                    .unwrap();
                println!("Wrote {path:?}");
            } else {
                println!("No --export <PATH> specified, not writing.");
            }
        },
    }
}

pub fn read (filename: &Path) -> Vec<u8> {
    let mut f      = File::open(&filename).expect("file not found");
    let metadata   = metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");
    buffer
}
