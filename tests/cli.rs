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

    let mut cmd = Command::cargo_bin("fast-transit-network-analytics").unwrap();
    cmd.args([
        "bfs",
        "--input",
        graph.path().to_str().unwrap(),
        "--source",
        "0",
        "--mode",
        "seq",
    ]);
    cmd.assert().success();
}

#[test]
fn cli_bfs_par_writes_output() {
    let graph = write_small_graph();

    let mut cmd = Command::cargo_bin("fast-transit-network-analytics").unwrap();
    cmd.args([
        "bfs",
        "--input",
        graph.path().to_str().unwrap(),
        "--source",
        "0",
        "--mode",
        "par",
    ]);
    cmd.assert().success();
}
