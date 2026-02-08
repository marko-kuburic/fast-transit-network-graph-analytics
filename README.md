# ftn-graph-analytics

This project provides a Rust CLI scaffold for large-scale graph analytics. Algorithms and data structures are implemented in subsequent commits.

## Build

```
cargo build --release
```

## Run

```
cargo run --release -- --help
```

## Datasets

Input graphs are plain text edge lists (one `src dst` pair per line). Datasets are not provided.

## Synthetic generator

Generate a directed Erdős–Rényi graph with deterministic output:

```
cargo run --release --bin gen -- --nodes 1000 --edges 5000 --seed 1 --model erdos-renyi --out data.txt
```
