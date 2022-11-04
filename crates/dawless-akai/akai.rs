#![feature(adt_const_params)]

macro_rules! module { ($name:ident) => { mod $name; pub use $name::*; }; }

module!(akai_string);
module!(akai_device);
module!(akai_disk);
module!(akai_file);
module!(akai_sample);

#[cfg(feature = "cli")]
pub(crate) use brailledump::BrailleDump;

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
