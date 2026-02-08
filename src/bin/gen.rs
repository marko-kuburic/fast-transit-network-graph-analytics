use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::{Parser, ValueEnum};
use rand::rngs::StdRng;
use rand::SeedableRng;

use fast_transit_network_analytics::generator::er::generate_erdos_renyi;

#[derive(Parser, Debug)]
#[command(name = "gen")]
#[command(about = "Synthetic graph generator", long_about = None)]
struct Cli {
    #[arg(long)]
    nodes: u32,
    #[arg(long)]
    edges: u64,
    #[arg(long)]
    seed: u64,
    #[arg(long)]
    out: PathBuf,
    #[arg(long, default_value = "erdos-renyi")]
    model: Model,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
enum Model {
    #[value(name = "erdos-renyi")]
    ErdosRenyi,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let file = File::create(&cli.out)
        .with_context(|| format!("failed to create output file: {}", cli.out.display()))?;
    let mut writer = BufWriter::new(file);
    let mut rng = StdRng::seed_from_u64(cli.seed);

    match cli.model {
        Model::ErdosRenyi => {
            generate_erdos_renyi(cli.nodes, cli.edges, &mut rng, &mut writer)?;
        }
    }
    writer.flush()?;
    Ok(())
}
