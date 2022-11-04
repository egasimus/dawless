use std::io::{Read, Write};
use std::fs::File;
use std::path::PathBuf;
use std::ops::Deref;
use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    brand: Brand,
}

#[derive(Subcommand)]
enum Brand {
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
    match &Cli::parse().brand {
        Brand::AKAI { model } => dawless_akai::cli(model),
        Brand::Korg { model } => dawless_korg::cli(model),
    };
}
