use std::sync::atomic::{AtomicU32, Ordering};

use anyhow::{anyhow, Result};
use rayon::prelude::*;

use crate::graph::CsrGraph;

pub fn wcc_par(graph: &CsrGraph) -> Result<Vec<u32>> {
    let n = graph.n;
    let mut out = vec![u32::MAX; n];
    if n == 0 {
        return Ok(out);
    }
    if graph.rev_offsets.is_none() {
        return Err(anyhow!("reverse CSR required for undirected WCC"));
    }

    let comp: Vec<AtomicU32> = (0..n).map(|_| AtomicU32::new(u32::MAX)).collect();

    for v in 0..n {
        if comp[v].load(Ordering::SeqCst) != u32::MAX {
            continue;
        }
        let comp_id = v as u32;
        comp[v].store(comp_id, Ordering::SeqCst);

        let mut frontier = vec![comp_id];
        while !frontier.is_empty() {
            let next = frontier
                .par_iter()
                .map(|&u| {
                    let mut local = Vec::new();
                    for &nbr in graph.out_neighbors(u) {
                        if comp[nbr as usize]
                            .compare_exchange(u32::MAX, comp_id, Ordering::SeqCst, Ordering::SeqCst)
                            .is_ok()
                        {
                            local.push(nbr);
                        }
                    }
                    if let Some(in_neighbors) = graph.in_neighbors(u) {
                        for &nbr in in_neighbors {
                            if comp[nbr as usize]
                                .compare_exchange(
                                    u32::MAX,
                                    comp_id,
                                    Ordering::SeqCst,
                                    Ordering::SeqCst,
                                )
                                .is_ok()
                            {
                                local.push(nbr);
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
    }

    for i in 0..n {
        out[i] = comp[i].load(Ordering::SeqCst);
    }

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::wcc_par;
    use crate::graph::CsrGraph;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn wcc_par_two_components() {
        let mut tmp = NamedTempFile::new().unwrap();
        writeln!(tmp, "0 1").unwrap();
        writeln!(tmp, "1 0").unwrap();
        writeln!(tmp, "2 3").unwrap();

        let g = CsrGraph::from_edge_list(tmp.path(), false, true).unwrap();
        let comp = wcc_par(&g).unwrap();
        assert_eq!(comp[0], 0);
        assert_eq!(comp[1], 0);
        assert_eq!(comp[2], 2);
        assert_eq!(comp[3], 2);
    }
}
