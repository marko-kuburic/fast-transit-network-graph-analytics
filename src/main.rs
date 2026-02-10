use std::fmt::Display;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::time::Instant;

use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};
use rayon::ThreadPoolBuilder;

use fast_transit_network_analytics::algorithms::{
    bfs_par, bfs_seq, pagerank_par, pagerank_seq, wcc_par, wcc_seq,
};
use fast_transit_network_analytics::graph::CsrGraph;

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
        #[arg(long)]
        threads: Option<usize>,
        #[arg(long)]
        out: PathBuf,
    },
    Wcc {
        #[arg(long)]
        input: PathBuf,
        #[arg(long, default_value = "seq")]
        mode: Mode,
        #[arg(long)]
        threads: Option<usize>,
        #[arg(long)]
        out: PathBuf,
    },
    Pagerank {
        #[arg(long)]
        input: PathBuf,
        #[arg(long, default_value = "seq")]
        mode: Mode,
        #[arg(long)]
        threads: Option<usize>,
        #[arg(long)]
        out: PathBuf,
        #[arg(long, default_value_t = 0.85)]
        alpha: f64,
        #[arg(long, default_value_t = 50)]
        iters: usize,
        #[arg(long, default_value_t = 1e-10)]
        eps: f64,
    },
}

#[derive(Copy, Clone, Debug, ValueEnum)]
enum Mode {
    Seq,
    Par,
}

fn write_vec<T: Display>(out: &PathBuf, values: &[T]) -> Result<()> {
    let file = File::create(out)?;
    let mut writer = BufWriter::new(file);
    for v in values {
        writeln!(writer, "{}", v)?;
    }
    Ok(())
}

fn run_with_threads<F, T>(threads: Option<usize>, f: F) -> Result<T>
where
    F: FnOnce() -> Result<T> + Send,
    T: Send,
{
    if let Some(t) = threads {
        let pool = ThreadPoolBuilder::new().num_threads(t).build()?;
        pool.install(f)
    } else {
        f()
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Bfs {
            input,
            source,
            mode,
            threads,
            out,
        } => {
            let graph = CsrGraph::from_edge_list(&input, false, true)?;
            let start = Instant::now();
            let dist = match mode {
                Mode::Seq => bfs_seq::bfs_undirected(&graph, source),
                Mode::Par => run_with_threads(threads, || bfs_par::bfs_undirected_par(&graph, source))?,
            };
            let elapsed_ns = start.elapsed().as_nanos();
            eprintln!("ALGO_TIME_NS:{}", elapsed_ns);
            write_vec(&out, &dist)?;
        }
        Commands::Wcc {
            input,
            mode,
            threads,
            out,
        } => {
            let graph = CsrGraph::from_edge_list(&input, false, true)?;
            let start = Instant::now();
            let comp = match mode {
                Mode::Seq => wcc_seq::wcc(&graph),
                Mode::Par => run_with_threads(threads, || wcc_par::wcc_par(&graph))?,
            };
            let elapsed_ns = start.elapsed().as_nanos();
            eprintln!("ALGO_TIME_NS:{}", elapsed_ns);
            write_vec(&out, &comp)?;
        }
        Commands::Pagerank {
            input,
            mode,
            threads,
            out,
            alpha,
            iters,
            eps,
        } => {
            let graph = CsrGraph::from_edge_list(&input, false, true)?;
            let start = Instant::now();
            let ranks = match mode {
                Mode::Seq => pagerank_seq::pagerank(&graph, alpha, iters, eps),
                Mode::Par => run_with_threads(threads, || pagerank_par::pagerank_par(&graph, alpha, iters, eps))?,
            };
            let elapsed_ns = start.elapsed().as_nanos();
            eprintln!("ALGO_TIME_NS:{}", elapsed_ns);
            write_vec(&out, &ranks)?;
        }
    }

    Ok(())
}
