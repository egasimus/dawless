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
            pub fn read (filename: &Path) -> Vec<u8> {
                let mut f      = File::open(&filename).expect("file not found");
                let metadata   = metadata(&filename).expect("unable to read metadata");
                let mut buffer = vec![0; metadata.len() as usize];
                f.read(&mut buffer).expect("buffer overflow");
                buffer
            }
            $($body)*
        }
    }
}

#[macro_export] macro_rules! tui {
    ($($body:item)*) => {
        #[cfg(feature = "tui")] pub use tui::*;
        #[cfg(feature = "tui")] mod tui {
            use super::*;
            use std::fs::{File, metadata};
            use std::io::{Read, Write};
            use std::path::{Path, PathBuf};
            use crossterm::event::KeyCode;
            pub fn read (filename: &Path) -> Vec<u8> {
                let mut f      = File::open(&filename).expect("file not found");
                let metadata   = metadata(&filename).expect("unable to read metadata");
                let mut buffer = vec![0; metadata.len() as usize];
                f.read(&mut buffer).expect("buffer overflow");
                buffer
            }
            $($body)*
        }
    }
}
