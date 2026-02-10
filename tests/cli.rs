use std::io::Write;

use assert_cmd::Command;
use tempfile::NamedTempFile;


fn write_small_graph() -> NamedTempFile {
    let mut tmp = NamedTempFile::new().unwrap();
    writeln!(tmp, "0 1").unwrap();
    writeln!(tmp, "1 2").unwrap();
    writeln!(tmp, "2 0").unwrap();
    tmp
}

#[test]
fn cli_bfs_seq_writes_output() {
    let graph = write_small_graph();
    let out = NamedTempFile::new().unwrap();

    let mut cmd = Command::cargo_bin("fast-transit-network-analytics").unwrap();
    cmd.args([
        "bfs",
        "--input",
        graph.path().to_str().unwrap(),
        "--source",
        "0",
        "--mode",
        "seq",
        "--out",
        out.path().to_str().unwrap(),
    ]);
    cmd.assert().success();
}

#[test]
fn cli_wcc_seq_runs() {
    let graph = write_small_graph();
    let out = NamedTempFile::new().unwrap();

    let mut cmd = Command::cargo_bin("fast-transit-network-analytics").unwrap();
    cmd.args([
        "wcc",
        "--input",
        graph.path().to_str().unwrap(),
        "--mode",
        "seq",
        "--out",
        out.path().to_str().unwrap(),
    ]);
    cmd.assert().success();
}

#[test]
fn cli_wcc_par_runs() {
    let graph = write_small_graph();
    let out = NamedTempFile::new().unwrap();

    let mut cmd = Command::cargo_bin("fast-transit-network-analytics").unwrap();
    cmd.args([
        "wcc",
        "--input",
        graph.path().to_str().unwrap(),
        "--mode",
        "par",
        "--threads",
        "2",
        "--out",
        out.path().to_str().unwrap(),
    ]);
    cmd.assert().success();
}

#[test]
fn cli_bfs_par_writes_output() {
    let graph = write_small_graph();
    let out = NamedTempFile::new().unwrap();

    let mut cmd = Command::cargo_bin("fast-transit-network-analytics").unwrap();
    cmd.args([
        "bfs",
        "--input",
        graph.path().to_str().unwrap(),
        "--source",
        "0",
        "--mode",
        "par",
        "--threads",
        "2",
        "--out",
        out.path().to_str().unwrap(),
    ]);
    cmd.assert().success();
}

#[test]
fn cli_pagerank_seq_runs() {
    let graph = write_small_graph();
    let out = NamedTempFile::new().unwrap();

    let mut cmd = Command::cargo_bin("fast-transit-network-analytics").unwrap();
    cmd.args([
        "pagerank",
        "--input",
        graph.path().to_str().unwrap(),
        "--mode",
        "seq",
        "--out",
        out.path().to_str().unwrap(),
        "--alpha",
        "0.85",
        "--iters",
        "10",
        "--eps",
        "1e-8",
    ]);
    cmd.assert().success();
}

#[test]
fn cli_pagerank_par_runs() {
    let graph = write_small_graph();
    let out = NamedTempFile::new().unwrap();

    let mut cmd = Command::cargo_bin("fast-transit-network-analytics").unwrap();
    cmd.args([
        "pagerank",
        "--input",
        graph.path().to_str().unwrap(),
        "--mode",
        "par",
        "--threads",
        "2",
        "--out",
        out.path().to_str().unwrap(),
        "--alpha",
        "0.85",
        "--iters",
        "10",
        "--eps",
        "1e-8",
    ]);
    cmd.assert().success();
}

