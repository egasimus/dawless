#[derive(clap::Subcommand)]
pub enum Korg {

    /// Tools for the Korg Electribe 2
    Electribe2 {
        #[command(subcommand)]
        command: crate::electribe2::Electribe2CLI,
    },

    ///// Tools for the Korg Triton
    //Triton {
        ///// Import an existing disk image
        //#[clap(long)]
        //command: Triton,
    //},

}

pub fn run_cli (device: &Korg) {
    match device {
        Korg::Electribe2 { command } => crate::electribe2::cli(command),
        //Korg::Triton{ command } => triton::cli(command),
    }
}
