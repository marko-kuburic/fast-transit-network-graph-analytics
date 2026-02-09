use std::collections::VecDeque;

use crate::graph::CsrGraph;

pub fn bfs_undirected(graph: &CsrGraph, source: u32) -> Vec<i64> {
    let n = graph.n;
    let mut dist = vec![-1i64; n];
    if n == 0 {
        return dist;
    }
    if source as usize >= n {
        return dist;
    }

    let mut queue = VecDeque::new();
    dist[source as usize] = 0;
    queue.push_back(source);

    while let Some(u) = queue.pop_front() {
        let du = dist[u as usize];
        for &v in graph.out_neighbors(u) {
            let vi = v as usize;
            if dist[vi] == -1 {
                dist[vi] = du + 1;
                queue.push_back(v);
            }
        }
        if let Some(in_neighbors) = graph.in_neighbors(u) {
            for &v in in_neighbors {
                let vi = v as usize;
                if dist[vi] == -1 {
                    dist[vi] = du + 1;
                    queue.push_back(v);
                }
            }
        }
    }

    dist
}

#[cfg(test)]
mod tests {
    use super::bfs_undirected;
    use crate::graph::CsrGraph;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn bfs_small_graph() {
        let mut tmp = NamedTempFile::new().unwrap();
        writeln!(tmp, "0 1").unwrap();
        writeln!(tmp, "1 2").unwrap();
        writeln!(tmp, "2 3").unwrap();
        writeln!(tmp, "3 0").unwrap();

        let g = CsrGraph::from_edge_list(tmp.path(), false, true).unwrap();
        let dist = bfs_undirected(&g, 0);
        assert_eq!(dist, vec![0, 1, 2, 1]);
    }
}
