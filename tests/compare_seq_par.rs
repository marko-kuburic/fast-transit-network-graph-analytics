use rand::rngs::StdRng;
use rand::SeedableRng;
use tempfile::NamedTempFile;

use fast_transit_network_analytics::algorithms::{bfs_par, bfs_seq};
use fast_transit_network_analytics::generator::er::generate_erdos_renyi;
use fast_transit_network_analytics::graph::CsrGraph;

fn make_temp_graph(nodes: u32, edges: u64, seed: u64) -> NamedTempFile {
    let mut tmp = NamedTempFile::new().unwrap();
    let mut rng = StdRng::seed_from_u64(seed);
    generate_erdos_renyi(nodes, edges, &mut rng, &mut tmp).unwrap();
    tmp
}

#[test]
fn bfs_seq_par_equal() {
    let tmp = make_temp_graph(16, 64, 42);
    let g = CsrGraph::from_edge_list(tmp.path(), false, true).unwrap();
    let seq = bfs_seq::bfs_undirected(&g, 0);
    let par = bfs_par::bfs_undirected_par(&g, 0).unwrap();
    assert_eq!(seq, par);
}
