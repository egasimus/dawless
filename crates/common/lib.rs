#![feature(unboxed_closures)]
#![feature(fn_traits)]
#![feature(unsized_locals)]

//use std::path::PathBuf;
//use std::env::{current_dir, set_current_dir};
//use std::fs::{read_dir, metadata};
//use pathdiff::diff_paths;

mod tui;
pub use tui::*;

mod file;
pub use file::*;

#[macro_export] macro_rules! module {
    ($name:ident) => {
        mod $name;
        pub use $name::*;
    };
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
