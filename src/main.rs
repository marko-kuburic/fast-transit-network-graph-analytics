use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
#[command(name = "fast-transit-network-analytics")]
#[command(about = "Graph analytics on large edge lists", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Bfs {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        source: u32,
        #[arg(long, default_value = "seq")]
        mode: Mode,
    },
    Wcc {
        #[arg(long)]
        input: PathBuf,
        #[arg(long, default_value = "seq")]
        mode: Mode,
    },
    Pagerank {
        #[arg(long)]
        input: PathBuf,
        #[arg(long, default_value = "seq")]
        mode: Mode,
    },
}

#[derive(Copy, Clone, Debug, ValueEnum)]
enum Mode {
    Seq,
    Par,
}

fn main() -> Result<()> {
    let _cli = Cli::parse();
    Ok(())
}
