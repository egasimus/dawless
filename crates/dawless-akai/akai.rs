#![feature(adt_const_params)]
#[macro_use] extern crate dawless_common;
macro_rules! module { ($name:ident) => { mod $name; pub use $name::*; }; }

module!(akai_string);
module!(akai_device);
module!(akai_disk);
module!(akai_file);
module!(akai_sample);

#[cfg(feature = "cli")]
use std::io::{Read, Write};

#[cfg(feature = "cli")]
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

cli! {

    pub fn cli (model: &AKAI) {
        match model {
            AKAI::S3000 { import, sample, export, .. } => {
                let mut disk = akai_s3000().blank_disk();
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

}

use dawless_common::*;

pub struct MPC2000TUI {
    menu: List<()>
}
impl MPC2000TUI {
    pub fn new () -> Self {
        let mut menu = List::default();
        menu.add("Edit sample".into(), ());
        Self { menu }
    }
}
impl TUI for MPC2000TUI {
    fn size (&self) -> Size {
        self.menu.size()
    }
    fn render (&self, term: &mut dyn Write, space: &Space) -> Result<()> {
        Frame { title: "MPC2000".into(), ..Frame::default() }
            .render(term, space)?;
        self.menu.render(term, &space.inset(1))
    }
}

pub struct S3000XLTUI {
    menu: List<()>
}
impl S3000XLTUI {
    pub fn new () -> Self {
        let mut menu = List::default();
        menu.add("Edit sample".into(), ());
        Self { menu }
    }
}
impl TUI for S3000XLTUI {
    fn size (&self) -> Size {
        self.menu.size()
    }
    fn render (&self, term: &mut dyn Write, space: &Space) -> Result<()> {
        Frame { title: "S3000XL".into(), ..Frame::default() }
            .render(term, space)?;
        self.menu.render(term, &space.inset(1))
    }
}
