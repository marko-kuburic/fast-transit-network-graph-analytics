use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::{Parser, ValueEnum};
use rand::rngs::StdRng;
use rand::SeedableRng;
use rand::Rng;
use rayon::prelude::*;
use rayon::ThreadPoolBuilder;

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
            let threads = std::thread::available_parallelism()
                .map(|n| n.get())
                .unwrap_or(1);
            if threads <= 1 || cli.edges < 10000 {
                generate_erdos_renyi(cli.nodes, cli.edges, &mut rng, &mut writer)?;
            } else {
                let pool = ThreadPoolBuilder::new().num_threads(threads).build()?;
                let buffers = pool.install(|| {
                    let base = cli.edges / threads as u64;
                    let rem = cli.edges % threads as u64;
                    (0..threads)
                        .into_par_iter()
                        .map(|i| {
                            let count = base + if (i as u64) < rem { 1 } else { 0 };
                            let mut local_rng = StdRng::seed_from_u64(cli.seed ^ (i as u64 + 0x9e3779b97f4a7c15));
                            let mut buf = String::new();
                            for _ in 0..count {
                                let src = local_rng.gen_range(0..cli.nodes);
                                let dst = local_rng.gen_range(0..cli.nodes);
                                buf.push_str(&format!("{} {}\n", src, dst));
                            }
                            buf
                        })
                        .collect::<Vec<String>>()
                });
                for buf in buffers {
                    writer.write_all(buf.as_bytes())?;
                }
            }
        }
    }
    writer.flush()?;
    Ok(())
}
