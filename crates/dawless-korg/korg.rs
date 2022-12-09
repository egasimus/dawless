macro_rules! module { ($name:ident) => { mod $name; pub use $name::*; }; }

module!(electribe2);
module!(korg_triton);

#[cfg(feature = "cli")]
#[derive(clap::Subcommand)]
pub enum Korg {

    /// Tools for the Korg Electribe 2
    Electribe2 {
        #[command(subcommand)]
        command: Electribe2CLI,
    },

    ///// Tools for the Korg Triton
    //Triton {
        ///// Import an existing disk image
        //#[clap(long)]
        //command: Triton,
    //},

}

#[cfg(feature = "cli")]
pub fn cli (model: &Korg) {
    match model {
        Korg::Electribe2 { command } => electribe2::cli(command),
        //Korg::Triton{ command } => triton::cli(command),
    }
}
