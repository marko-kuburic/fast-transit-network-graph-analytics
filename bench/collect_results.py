import csv
from collections import defaultdict
from pathlib import Path

import matplotlib.pyplot as plt

RESULTS_PATH = Path("bench/results.csv")
FIG_DIR = Path("bench/figures")
FIG_DIR.mkdir(parents=True, exist_ok=True)


def input_label(path_str: str) -> str:
    name = Path(path_str).name
    if "small" in name:
        return "small"
    if "large" in name:
        return "large"
    return name


rows = []
with RESULTS_PATH.open(newline="") as f:
    for row in csv.DictReader(f):
        rows.append(row)

# Normalize and group rows
records = []
for r in rows:
    mode = r["mode"]
    algo = r["algo"]
    input_path = r["input"]
    threads = r.get("threads") or ""
    dur_ms = int(r.get("duration_ms") or 0)
    if dur_ms == 0:
        dur_ms = int(r["duration_ns"]) // 1_000_000
    records.append(
        {
            "algo": algo,
            "mode": mode,
            "input": input_path,
            "input_label": input_label(input_path),
            "threads": int(threads) if threads else None,
            "duration_ms": dur_ms,
        }
    )

baselines = {}
for r in records:
    if r["mode"] == "seq":
        baselines[(r["algo"], r["input"])] = r["duration_ms"]

# Speedups by algo/input/threads
speedups = defaultdict(list)
for r in records:
    if r["mode"] == "par":
        base = baselines.get((r["algo"], r["input"]))
        if base:
            speedups[(r["algo"], r["input"], r["threads"])].append(base / r["duration_ms"])

# Print summary speedups
for (algo, input_path, threads), values in sorted(speedups.items()):
    avg = sum(values) / len(values)
    print(f"{algo},{input_path},{threads},avg_speedup,{avg:.3f}")


def plot_runtime_by_threads():
    grouped = defaultdict(list)
    for r in records:
        if r["mode"] == "par" and r["threads"] is not None:
            grouped[(r["algo"], r["input_label"])].append(r)

    for (algo, size), vals in grouped.items():
        vals.sort(key=lambda x: x["threads"])
        xs = [v["threads"] for v in vals]
        ys = [v["duration_ms"] for v in vals]
        plt.figure()
        plt.plot(xs, ys, marker="o")
        plt.title(f"{algo.upper()} parallel runtime ({size})")
        plt.xlabel("threads")
        plt.ylabel("duration (ms)")
        plt.grid(True, alpha=0.3)
        out = FIG_DIR / f"runtime_{algo}_{size}.png"
        plt.savefig(out, dpi=150, bbox_inches="tight")
        plt.close()


def plot_speedup_by_threads():
    grouped = defaultdict(list)
    for (algo, input_path, threads), values in speedups.items():
        grouped[(algo, input_label(input_path))].append((threads, sum(values) / len(values)))

    for (algo, size), vals in grouped.items():
        vals.sort(key=lambda x: x[0])
        xs = [v[0] for v in vals]
        ys = [v[1] for v in vals]
        plt.figure()
        plt.plot(xs, ys, marker="o")
        plt.title(f"{algo.upper()} speedup vs threads ({size})")
        plt.xlabel("threads")
        plt.ylabel("speedup")
        plt.grid(True, alpha=0.3)
        out = FIG_DIR / f"speedup_{algo}_{size}.png"
        plt.savefig(out, dpi=150, bbox_inches="tight")
        plt.close()


def plot_seq_vs_par():
    # Compare seq vs best par for each algo, plotted separately by size
    best_par = {}
    for r in records:
        if r["mode"] == "par":
            key = (r["algo"], r["input_label"])
            best_par[key] = min(best_par.get(key, r["duration_ms"]), r["duration_ms"])

    seq = {}
    for r in records:
        if r["mode"] == "seq":
            seq[(r["algo"], r["input_label"])] = r["duration_ms"]

    pairs = sorted(set(seq.keys()) & set(best_par.keys()))
    if not pairs:
        return

    sizes = sorted({size for (_, size) in pairs})
    for size in sizes:
        size_pairs = [(algo, s) for (algo, s) in pairs if s == size]
        if not size_pairs:
            continue
        labels = [algo for (algo, _) in size_pairs]
        seq_vals = [seq[k] for k in size_pairs]
        par_vals = [best_par[k] for k in size_pairs]

        x = list(range(len(size_pairs)))
        width = 0.35
        plt.figure(figsize=(max(6, len(size_pairs) * 0.8), 4))
        plt.bar([i - width / 2 for i in x], seq_vals, width, label="seq")
        plt.bar([i + width / 2 for i in x], par_vals, width, label="best par")
        plt.xticks(x, labels, rotation=30, ha="right")
        plt.ylabel("duration (ms)")
        plt.title(f"Seq vs best parallel ({size})")
        plt.legend()
        plt.grid(True, axis="y", alpha=0.3)
        out = FIG_DIR / f"seq_vs_par_{size}.png"
        plt.savefig(out, dpi=150, bbox_inches="tight")
        plt.close()


plot_runtime_by_threads()
plot_speedup_by_threads()
plot_seq_vs_par()
