use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about = "A CLI tool to normalize seismological, meteorological, and astronomical data provided by various institutions.", long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

impl Default for Cli {
    fn default() -> Self {
        Self::parse()
    }
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Sei(SeiArgs),
    Met(MetArgs),
    Ast(AstArgs),
}

#[derive(Args, Debug)]
#[command(version = "0.1.0", about = "Seismology Mode", long_about = None)]
pub struct SeiArgs {
    /// Pass the path to the directory or file you wish to convert.
    #[arg(short, long)]
    path: PathBuf,
}

#[derive(Args, Debug)]
#[command(version = "0.1.0", about = "Meteorology Mode", long_about = None)]
pub struct MetArgs {
    /// Pass the path to the directory or file you wish to convert.
    #[arg(short, long)]
    path: PathBuf,
}

#[derive(Args, Debug)]
#[command(version = "0.1.0", about = "Astronomy Mode", long_about = None)]
pub struct AstArgs {
    /// Pass the path to the directory or file you wish to convert.
    #[arg(short, long)]
    path: PathBuf,
}
