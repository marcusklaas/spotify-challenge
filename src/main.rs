extern crate spotify;

use spotify::voter_input;
use spotify::bipartite_matchings;
use spotify::bipartite_matchings::BipartiteGraph;

fn main() {
    let (cat_count, dog_count, voter_count) = match voter_input::get_parameters() {
        None         => fail!("Incorrect parameters"),
        Some(triple) => triple
    };
    
    let (dog_lovers, cat_lovers) = voter_input::get_voter_list(voter_count);
    
    let graph = BipartiteGraph::from_closure(
        &mut dog_lovers.iter(), 
        &mut cat_lovers.iter(),
        |dog, cat| ! dog.is_compatible(*cat)
    );
    
    let maximum_matching_size = bipartite_matchings::get_max_matching_size(&graph);
    
    println!("{}", dog_lovers.len() + cat_lovers.len() - maximum_matching_size);
}
