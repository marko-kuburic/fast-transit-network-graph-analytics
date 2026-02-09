use std::collections::{BTreeSet, HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use anyhow::{anyhow, Context, Result};

#[derive(Clone, Debug)]
pub struct CsrGraph {
    pub n: usize,
    pub m: usize,
    pub offsets: Vec<usize>,
    pub edges: Vec<u32>,
    pub rev_offsets: Option<Vec<usize>>,
    pub rev_edges: Option<Vec<u32>>,
}

impl CsrGraph {
    pub fn from_edge_list(path: &Path, remap: bool, build_rev: bool) -> Result<Self> {
        if remap {
            Self::from_edge_list_remap(path, build_rev)
        } else {
            Self::from_edge_list_direct(path, build_rev)
        }
    }

    pub fn out_neighbors(&self, v: u32) -> &[u32] {
        let v = v as usize;
        let start = self.offsets[v];
        let end = self.offsets[v + 1];
        &self.edges[start..end]
    }

    pub fn in_neighbors(&self, v: u32) -> Option<&[u32]> {
        let offsets = self.rev_offsets.as_ref()?;
        let edges = self.rev_edges.as_ref()?;
        let v = v as usize;
        let start = offsets[v];
        let end = offsets[v + 1];
        Some(&edges[start..end])
    }

    fn from_edge_list_direct(path: &Path, build_rev: bool) -> Result<Self> {
        let file = File::open(path)
            .with_context(|| format!("failed to open edge list: {}", path.display()))?;
        let reader = BufReader::new(file);

        let mut max_id: u64 = 0;
        let mut m: usize = 0;
        let mut degrees: Vec<usize> = Vec::new();
        let mut rev_degrees: Vec<usize> = Vec::new();

        for (line_no, line) in reader.lines().enumerate() {
            let line = line.context("failed to read line")?;
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            let (src, dst) = parse_edge(line)
                .with_context(|| format!("invalid edge at line {}", line_no + 1))?;
            if src > u32::MAX as u64 || dst > u32::MAX as u64 {
                return Err(anyhow!("vertex id exceeds u32::MAX"));
            }
            max_id = max_id.max(src).max(dst);

            let s = src as usize;
            if s >= degrees.len() {
                degrees.resize(s + 1, 0);
            }
            degrees[s] += 1;

            if build_rev {
                let d = dst as usize;
                if d >= rev_degrees.len() {
                    rev_degrees.resize(d + 1, 0);
                }
                rev_degrees[d] += 1;
            }
            m += 1;
        }

        let n = if m == 0 { 0 } else { (max_id + 1) as usize };
        degrees.resize(n, 0);
        if build_rev {
            rev_degrees.resize(n, 0);
        }

        let offsets = prefix_sum(&degrees);
        let mut edges = vec![0u32; m];
        let mut write_pos = offsets[..n].to_vec();

        let (mut rev_offsets, mut rev_edges, mut rev_write_pos) = if build_rev {
            let ro = prefix_sum(&rev_degrees);
            let re = vec![0u32; m];
            let rw = ro[..n].to_vec();
            (Some(ro), Some(re), Some(rw))
        } else {
            (None, None, None)
        };

        let file = File::open(path)
            .with_context(|| format!("failed to reopen edge list: {}", path.display()))?;
        let reader = BufReader::new(file);

        for (line_no, line) in reader.lines().enumerate() {
            let line = line.context("failed to read line")?;
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            let (src, dst) = parse_edge(line)
                .with_context(|| format!("invalid edge at line {}", line_no + 1))?;
            let s = src as usize;
            let d = dst as u32;
            let pos = write_pos[s];
            edges[pos] = d;
            write_pos[s] += 1;

            if let (Some(ref mut re), Some(ref mut rw)) =
                (rev_edges.as_mut(), rev_write_pos.as_mut())
            {
                let di = dst as usize;
                let rpos = rw[di];
                re[rpos] = src as u32;
                rw[di] += 1;
            }
        }

        Ok(CsrGraph {
            n,
            m,
            offsets,
            edges,
            rev_offsets,
            rev_edges,
        })
    }

    fn from_edge_list_remap(path: &Path, build_rev: bool) -> Result<Self> {
        let file = File::open(path)
            .with_context(|| format!("failed to open edge list: {}", path.display()))?;
        let reader = BufReader::new(file);

        let mut ids: BTreeSet<u64> = BTreeSet::new();
        let mut edges_raw: Vec<(u64, u64)> = Vec::new();

        for (line_no, line) in reader.lines().enumerate() {
            let line = line.context("failed to read line")?;
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            let (src, dst) = parse_edge(line)
                .with_context(|| format!("invalid edge at line {}", line_no + 1))?;
            ids.insert(src);
            ids.insert(dst);
            edges_raw.push((src, dst));
        }

        let n = ids.len();
        if n > u32::MAX as usize {
            return Err(anyhow!("remapped vertex count exceeds u32::MAX"));
        }

        let mut mapping: HashMap<u64, u32> = HashMap::with_capacity(n * 2);
        for (i, id) in ids.into_iter().enumerate() {
            mapping.insert(id, i as u32);
        }

        let mut degrees = vec![0usize; n];
        let mut rev_degrees = vec![0usize; n];
        for (src, dst) in edges_raw.iter().copied() {
            let s = *mapping.get(&src).unwrap() as usize;
            let d = *mapping.get(&dst).unwrap() as usize;
            degrees[s] += 1;
            if build_rev {
                rev_degrees[d] += 1;
            }
        }

        let offsets = prefix_sum(&degrees);
        let mut edges = vec![0u32; edges_raw.len()];
        let mut write_pos = offsets[..n].to_vec();

        let (rev_offsets, mut rev_edges, mut rev_write_pos) = if build_rev {
            let ro = prefix_sum(&rev_degrees);
            let re = vec![0u32; edges_raw.len()];
            let rw = ro[..n].to_vec();
            (Some(ro), Some(re), Some(rw))
        } else {
            (None, None, None)
        };

        for (src, dst) in edges_raw {
            let s = *mapping.get(&src).unwrap() as usize;
            let d = *mapping.get(&dst).unwrap() as u32;
            let pos = write_pos[s];
            edges[pos] = d;
            write_pos[s] += 1;

            if let (Some(ref mut re), Some(ref mut rw)) =
                (rev_edges.as_mut(), rev_write_pos.as_mut())
            {
                let di = d as usize;
                let rpos = rw[di];
                re[rpos] = *mapping.get(&src).unwrap();
                rw[di] += 1;
            }
        }

        Ok(CsrGraph {
            n,
            m: edges.len(),
            offsets,
            edges,
            rev_offsets,
            rev_edges,
        })
    }
}

fn parse_edge(line: &str) -> Result<(u64, u64)> {
    let mut it = line.split_whitespace();
    let src = it
        .next()
        .ok_or_else(|| anyhow!("missing src"))?
        .parse::<u64>()
        .context("invalid src")?;
    let dst = it
        .next()
        .ok_or_else(|| anyhow!("missing dst"))?
        .parse::<u64>()
        .context("invalid dst")?;
    Ok((src, dst))
}

fn prefix_sum(degrees: &[usize]) -> Vec<usize> {
    let mut offsets = vec![0usize; degrees.len() + 1];
    let mut sum = 0usize;
    for (i, &d) in degrees.iter().enumerate() {
        offsets[i] = sum;
        sum += d;
    }
    offsets[degrees.len()] = sum;
    offsets
}

#[cfg(test)]
mod tests {
    use super::CsrGraph;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn csr_construction_direct() {
        let mut tmp = NamedTempFile::new().unwrap();
        writeln!(tmp, "0 1").unwrap();
        writeln!(tmp, "0 2").unwrap();
        writeln!(tmp, "1 2").unwrap();
        writeln!(tmp, "2 0").unwrap();

        let g = CsrGraph::from_edge_list(tmp.path(), false, true).unwrap();
        assert_eq!(g.n, 3);
        assert_eq!(g.m, 4);
        assert_eq!(g.offsets, vec![0, 2, 3, 4]);
        assert_eq!(g.edges, vec![1, 2, 2, 0]);
        assert!(g.rev_offsets.is_some());
    }

    #[test]
    fn csr_remap() {
        let mut tmp = NamedTempFile::new().unwrap();
        writeln!(tmp, "10 20").unwrap();
        writeln!(tmp, "20 10").unwrap();

        let g = CsrGraph::from_edge_list(tmp.path(), true, false).unwrap();
        assert_eq!(g.n, 2);
        assert_eq!(g.m, 2);
        assert_eq!(g.offsets, vec![0, 1, 2]);
        assert_eq!(g.edges, vec![1, 0]);
    }

    #[test]
    fn csr_from_bench_file() {
        let path = std::path::Path::new("bench/sample_small.txt");
        let g = CsrGraph::from_edge_list(path, false, true).unwrap();
        assert_eq!(g.n, 5);
        assert_eq!(g.m, 6);
        let path = std::path::Path::new("bench/sample_big.txt");
        let g = CsrGraph::from_edge_list(path, false, true).unwrap();
        assert_eq!(g.n, 10);
        assert_eq!(g.m, 20);
    }
}
