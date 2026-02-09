use std::sync::atomic::{AtomicI64, Ordering};

use anyhow::{anyhow, Result};
use rayon::prelude::*;

use crate::graph::CsrGraph;

pub fn bfs_undirected_par(graph: &CsrGraph, source: u32) -> Result<Vec<i64>> {
    let n = graph.n;
    let mut dist_out = vec![-1i64; n];
    if n == 0 {
        return Ok(dist_out);
    }
    if source as usize >= n {
        return Ok(dist_out);
    }
    if graph.rev_offsets.is_none() {
        return Err(anyhow!("reverse CSR required for undirected BFS"));
    }

    let dist: Vec<AtomicI64> = (0..n).map(|_| AtomicI64::new(-1)).collect();
    dist[source as usize].store(0, Ordering::SeqCst);

    let mut frontier = vec![source];
    let mut level: i64 = 0;

    while !frontier.is_empty() {
        level += 1;
        let next = frontier
            .par_iter()
            .map(|&u| {
                let mut local = Vec::new();
                for &v in graph.out_neighbors(u) {
                    if dist[v as usize]
                        .compare_exchange(-1, level, Ordering::SeqCst, Ordering::SeqCst)
                        .is_ok()
                    {
                        local.push(v);
                    }
                }
                if let Some(in_neighbors) = graph.in_neighbors(u) {
                    for &v in in_neighbors {
                        if dist[v as usize]
                            .compare_exchange(-1, level, Ordering::SeqCst, Ordering::SeqCst)
                            .is_ok()
                        {
                            local.push(v);
                        }
                    }
                }
                local
            })
            .reduce(Vec::new, |mut a, b| {
                a.extend(b);
                a
            });
        frontier = next;
    }

    for i in 0..n {
        dist_out[i] = dist[i].load(Ordering::SeqCst);
    }
    Ok(dist_out)
}

#[cfg(test)]
mod tests {
    use super::bfs_undirected_par;
    use crate::graph::CsrGraph;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn bfs_par_matches_seq() {
        let mut tmp = NamedTempFile::new().unwrap();
        writeln!(tmp, "0 1").unwrap();
        writeln!(tmp, "1 2").unwrap();
        writeln!(tmp, "2 3").unwrap();
        writeln!(tmp, "3 0").unwrap();

        let g = CsrGraph::from_edge_list(tmp.path(), false, true).unwrap();
        let dist = bfs_undirected_par(&g, 0).unwrap();
        assert_eq!(dist, vec![0, 1, 2, 1]);
    }
}
