extern crate spotify;

use spotify::bipartite_matchings::BipartiteGraph;

#[test]
fn test_graph_creation() {
    let rows = &[0u, ..10];
    let columns = &[0u, ..10];

    let graph = BipartiteGraph::from_closure(rows, columns, |x,y| true);
    
    assert_eq!(graph.get_max_matching_size(), 10);
}
