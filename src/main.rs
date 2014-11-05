extern crate spotify;

use spotify::voter_input;
use spotify::voter_input::Voter;

use spotify::bipartite_matchings;

fn main() {
    let (cat_count, dog_count, voter_count) = match voter_input::get_parameters() {
        Err(why) => fail!(why),
        Ok(triple) => triple
    };
    
    let (dog_lovers, cat_lovers) = voter_input::get_voter_list(voter_count);
    
    let mut graph = bipartite_matchings::BipartiteGraph::new(dog_lovers.len(), cat_lovers.len());
    
    for (dog_index, dog) in dog_lovers.iter().enumerate() {
        for (cat_index, cat) in cat_lovers.iter().enumerate() {
            graph.set_edge(dog_index, cat_index, !dog.is_compatible(cat));
        }
    }
    
    let maximum_matching_size = bipartite_matchings::get_max_matching_size(&graph);
    
    println!("{}", dog_lovers.len() + cat_lovers.len() - maximum_matching_size);
}
