use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    #[cfg(feature="tui")]
    /// Launch the interactive interface
    TUI,
    #[cfg(feature="cli")]
    /// Tools for AKAI devices
    AKAI {
        #[command(subcommand)]
        model: dawless_akai::AKAI,
    },
    #[cfg(feature="cli")]
    /// Tools for Korg devices
    Korg {
        #[command(subcommand)]
        model: dawless_korg::Korg,
    },
}

pub fn main () {
    match &Cli::parse().command {
        #[cfg(feature="cli")]
        Command::AKAI { model } => dawless_akai::run_cli(model),
        #[cfg(feature="cli")]
        Command::Korg { model } => dawless_korg::run_cli(model),
        #[cfg(feature="tui")]
        Command::TUI => crate::tui::main().unwrap(),
    };
}
