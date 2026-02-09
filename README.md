# fast-transit-network-analytics

This project provides a Rust CLI scaffold for large-scale graph analytics, with a CSR graph representation and BFS implementations (sequential and parallel).

## Requirements

- Rust 1.80+ (CI uses 1.80)

## Build

```
cargo build --release
```

## Run

```
cargo run --release -- --help
```

### BFS

Sequential BFS:

```
cargo run --release -- bfs --input data.txt --source 0 --mode seq
```

Parallel BFS:

```
cargo run --release -- bfs --input data.txt --source 0 --mode par
```

## Datasets

Input graphs are plain text edge lists (one `src dst` pair per line). Datasets are not provided.

## Synthetic generator

Generate a directed Erdős–Rényi graph with deterministic output:

```
cargo run --release --bin gen -- --nodes 1000 --edges 5000 --seed 1 --model erdos-renyi --out data.txt
```

## Tests

```
cargo test
```
