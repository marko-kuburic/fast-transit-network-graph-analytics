use std::collections::VecDeque;

use crate::graph::CsrGraph;

pub fn wcc(graph: &CsrGraph) -> Vec<u32> {
    let n = graph.n;
    let mut comp = vec![u32::MAX; n];

    for v in 0..n {
        if comp[v] != u32::MAX {
            continue;
        }
        let comp_id = v as u32;
        let mut queue = VecDeque::new();
        comp[v] = comp_id;
        queue.push_back(v as u32);

        while let Some(u) = queue.pop_front() {
            for &nbr in graph.out_neighbors(u) {
                let ni = nbr as usize;
                if comp[ni] == u32::MAX {
                    comp[ni] = comp_id;
                    queue.push_back(nbr);
                }
            }
            if let Some(in_neighbors) = graph.in_neighbors(u) {
                for &nbr in in_neighbors {
                    let ni = nbr as usize;
                    if comp[ni] == u32::MAX {
                        comp[ni] = comp_id;
                        queue.push_back(nbr);
                    }
                }
            }
        }
    }

    comp
}

#[cfg(test)]
mod tests {
    use super::wcc;
    use crate::graph::CsrGraph;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn wcc_two_components() {
        let mut tmp = NamedTempFile::new().unwrap();
        writeln!(tmp, "0 1").unwrap();
        writeln!(tmp, "1 0").unwrap();
        writeln!(tmp, "2 3").unwrap();

        let g = CsrGraph::from_edge_list(tmp.path(), false, true).unwrap();
        let comp = wcc(&g);
        assert_eq!(comp[0], 0);
        assert_eq!(comp[1], 0);
        assert_eq!(comp[2], 2);
        assert_eq!(comp[3], 2);
    }
}
