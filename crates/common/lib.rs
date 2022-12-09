//use std::path::PathBuf;
//use std::env::{current_dir, set_current_dir};
//use std::fs::{read_dir, metadata};
//use pathdiff::diff_paths;

mod lib_tui;
pub use lib_tui::*;

#[macro_export] macro_rules! module {
    ($name:ident) => {
        mod $name;
        pub use $name::*;
    };
}

use std::fs::{File, metadata};
use std::io::{Read};
use std::path::{Path};
pub fn read (filename: &Path) -> Vec<u8> {
    let mut f      = File::open(&filename).expect("file not found");
    let metadata   = metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");
    buffer
}

#[macro_export] macro_rules! cli {
    ($($body:item)*) => {
        #[cfg(feature = "cli")] pub use cli::*;
        #[cfg(feature = "cli")] mod cli {
            use super::*;
            use std::fs::{File, metadata};
            use std::io::{Read, Write};
            use std::path::{Path, PathBuf};
            use dawless_common::read;
            $($body)*
        }
    }
}
