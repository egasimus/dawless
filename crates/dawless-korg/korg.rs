#[cfg(feature = "cli")]
#[derive(clap::Subcommand)]
pub enum Korg {

    /// Tools for the Korg Electribe 2
    Electribe2 {
        /// Import an existing disk image
        #[clap(long)]
        import:  Vec<std::path::PathBuf>,
    },

    /// Tools for the Korg Triton Rack
    TritonRack {
        /// Import an existing disk image
        #[clap(long)]
        import:  Vec<std::path::PathBuf>,
    },

}

#[cfg(feature = "cli")]
pub fn cli (model: &Korg) {
    unimplemented!()
}
