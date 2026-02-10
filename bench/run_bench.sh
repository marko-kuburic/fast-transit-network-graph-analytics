#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR=$(cd "$(dirname "$0")/.." && pwd)
BIN="$ROOT_DIR/target/release/fast-transit-network-analytics"
GEN="$ROOT_DIR/target/release/gen"

cargo build --release

OUT_CSV="$ROOT_DIR/bench/results.csv"
INPUT_SMALL="$ROOT_DIR/bench/net_graph_small.txt"
INPUT_LARGE="$ROOT_DIR/bench/net_graph_large.txt"

# Benchmark sizing defaults follow the Graph500 convention:
#   N = 2^SCALE vertices
#   M = edgefactor * N edges
# Reference: Graph500 Benchmark Specification (Kernel 2 BFS) sets edgefactor=16.
EDGEFACTOR=${EDGEFACTOR:-16}
SMALL_SCALE=${SMALL_SCALE:-18}
LARGE_SCALE=${LARGE_SCALE:-20}

# Allow overriding explicit node/edge counts via env vars.
SMALL_NODES=${SMALL_NODES:-$((1 << SMALL_SCALE))}
SMALL_EDGES=${SMALL_EDGES:-$((EDGEFACTOR * SMALL_NODES))}
LARGE_NODES=${LARGE_NODES:-$((1 << LARGE_SCALE))}
LARGE_EDGES=${LARGE_EDGES:-$((EDGEFACTOR * LARGE_NODES))}
SEED=${SEED:-1}
REGENERATE=${REGENERATE:-0}

if [[ "$REGENERATE" == "1" || ! -f "$INPUT_SMALL" || ! -f "$INPUT_LARGE" ]]; then
  if [[ ! -x "$GEN" ]]; then
    cargo build --release --bin gen
  fi
  "$GEN" --nodes "$SMALL_NODES" --edges "$SMALL_EDGES" --seed "$SEED" --model erdos-renyi --out "$INPUT_SMALL"
  "$GEN" --nodes "$LARGE_NODES" --edges "$LARGE_EDGES" --seed "$SEED" --model erdos-renyi --out "$INPUT_LARGE"
fi

run_case() {
  local algo=$1
  local mode=$2
  local threads=$3
  local input=$4

  local cmd=("$BIN" "$algo" --input "$input" --mode "$mode" --out /tmp/out.txt)
  if [[ "$algo" == "bfs" ]]; then
    cmd+=(--source 0)
  fi

  local output
  if [[ -n "$threads" ]]; then
    cmd+=(--threads "$threads")
    output=$(RAYON_NUM_THREADS="$threads" "${cmd[@]}" 2>&1 >/dev/null)
  else
    output=$("${cmd[@]}" 2>&1 >/dev/null)
  fi
  
  # Extract algorithm-only time from stderr output
  local dur_ns=$(echo "$output" | grep -oP 'ALGO_TIME_NS:\K[0-9]+')
  local dur_ms=$((dur_ns / 1000000))
  echo "$algo,$mode,$threads,$input,$dur_ns,$dur_ms" >> "$OUT_CSV"
}

rm -f "$OUT_CSV"

echo "algo,mode,threads,input,duration_ns,duration_ms" >> "$OUT_CSV"

for input in "$INPUT_SMALL" "$INPUT_LARGE"; do
  for algo in bfs wcc pagerank; do
    run_case "$algo" seq "" "$input"
    for t in 1 2 4 8 16 32; do
      run_case "$algo" par "$t" "$input"
    done
  done
done

echo "Results written to $OUT_CSV"

python3 "$ROOT_DIR/bench/collect_results.py"
