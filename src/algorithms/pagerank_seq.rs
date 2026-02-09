use crate::graph::CsrGraph;

pub fn pagerank(graph: &CsrGraph, alpha: f64, iters: usize, eps: f64) -> Vec<f64> {
    let n = graph.n;
    if n == 0 {
        return Vec::new();
    }
    let n_f = n as f64;
    let mut rank = vec![1.0 / n_f; n];
    let mut next = vec![0.0f64; n];

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

        for v in 0..n {
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
            next[v] = alpha * (acc + dangling_sum / n_f) + teleport;
        }

        let mut diff = 0.0f64;
        for i in 0..n {
            diff += (next[i] - rank[i]).abs();
            rank[i] = next[i];
        }
        if eps > 0.0 && diff < eps {
            break;
        }
    }

    rank
}

#[cfg(test)]
mod tests {
    use super::pagerank;
    use crate::graph::CsrGraph;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn pagerank_small_graph() {
        let mut tmp = NamedTempFile::new().unwrap();
        writeln!(tmp, "0 1").unwrap();
        writeln!(tmp, "1 2").unwrap();
        writeln!(tmp, "2 0").unwrap();

        let g = CsrGraph::from_edge_list(tmp.path(), false, true).unwrap();
        let r = pagerank(&g, 0.85, 50, 1e-8);
        let sum: f64 = r.iter().sum();
        assert!((sum - 1.0).abs() < 1e-6);
    }
}
