use anyhow::{anyhow, Result};
use rayon::prelude::*;

use crate::graph::CsrGraph;

pub fn pagerank_par(graph: &CsrGraph, alpha: f64, iters: usize, eps: f64) -> Result<Vec<f64>> {
    let n = graph.n;
    if n == 0 {
        return Ok(Vec::new());
    }
    if graph.rev_offsets.is_none() {
        return Err(anyhow!("reverse CSR required for PageRank"));
    }

    let n_f = n as f64;
    let mut rank = vec![1.0 / n_f; n];

    let out_deg: Vec<usize> = (0..n)
        .map(|i| graph.offsets[i + 1] - graph.offsets[i])
        .collect();

    for _ in 0..iters {
        let mut dangling_sum = 0.0f64;
        for i in 0..n {
            if out_deg[i] == 0 {
                dangling_sum += rank[i];
            }
        }

        let next: Vec<f64> = (0..n)
            .into_par_iter()
            .map(|v| {
                let mut acc = 0.0f64;
                if let Some(in_neighbors) = graph.in_neighbors(v as u32) {
                    for &u in in_neighbors {
                        let ui = u as usize;
                        let deg = out_deg[ui];
                        if deg > 0 {
                            acc += rank[ui] / deg as f64;
                        }
                    }
                }
                let teleport = (1.0 - alpha) / n_f;
                alpha * (acc + dangling_sum / n_f) + teleport
            })
            .collect();

        let mut diff = 0.0f64;
        for i in 0..n {
            diff += (next[i] - rank[i]).abs();
        }
        rank = next;
        if eps > 0.0 && diff < eps {
            break;
        }
    }

    Ok(rank)
}

#[cfg(test)]
mod tests {
    use super::pagerank_par;
    use crate::graph::CsrGraph;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn pagerank_par_small_graph() {
        let mut tmp = NamedTempFile::new().unwrap();
        writeln!(tmp, "0 1").unwrap();
        writeln!(tmp, "1 2").unwrap();
        writeln!(tmp, "2 0").unwrap();

        let g = CsrGraph::from_edge_list(tmp.path(), false, true).unwrap();
        let r = pagerank_par(&g, 0.85, 50, 1e-8).unwrap();
        let sum: f64 = r.iter().sum();
        assert!((sum - 1.0).abs() < 1e-6);
    }
}
