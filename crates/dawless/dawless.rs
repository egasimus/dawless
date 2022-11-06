use clap::{Parser, Subcommand};
mod tui;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Launch the interactive interface
    TUI,
    /// Tools for AKAI devices
    AKAI {
        #[command(subcommand)]
        model: dawless_akai::AKAI,
    },
    /// Tools for Korg devices
    Korg {
        #[command(subcommand)]
        model: dawless_korg::Korg,
    },
}

fn main () {
    match &Cli::parse().command {
        Command::TUI => tui::main().unwrap(),
        Command::AKAI { model } => dawless_akai::cli(model),
        Command::Korg { model } => dawless_korg::cli(model),
    };
}
