extern crate spotify;

use spotify::bipartite_matchings::BipartiteGraph;

#[test]
fn test_graph_creation() {
    let mut rows = range(0, 10i);
    let mut columns = range(0u, 10);

    let graph = BipartiteGraph::from_closure(&mut rows, &mut columns, |x,y| true);
    
    assert_eq!(graph.get_max_matching_size(), 10);
}
