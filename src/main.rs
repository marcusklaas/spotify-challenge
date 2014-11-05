extern crate spotify;

use spotify::voter_input;
use spotify::voter_input::Voter;

use spotify::bipartite_matchings;

fn main() {
    let (cat_count, dog_count, voter_count) = match voter_input::get_parameters() {
        Err(why) => fail!(why),
        Ok(triple) => triple
    };
    
    let voter_list: Vec<voter_input::Voter> = voter_input::get_voter_list(voter_count);
    
    let dog_lovers: Vec<Voter> = voter_list.iter().filter(|&x| !x.is_cat_person()).map(|&x| x).collect();
    let cat_lovers: Vec<Voter> = voter_list.iter().filter(|&x| x.is_cat_person()).map(|&x| x).collect();
    
    let mut graph = bipartite_matchings::BipartiteGraph::new(dog_lovers.len(), cat_lovers.len());
    
    for (dog_index, dog) in dog_lovers.iter().enumerate() {
        for (cat_index, cat) in cat_lovers.iter().enumerate() {
            graph.set_edge(dog_index, cat_index, !dog.is_compatible(cat));
        }
    }
    
    let maximum_matching_size = bipartite_matchings::get_max_matching_size(&graph);
    
    println!("Maximum satisfied voters: {}", voter_list.len() - maximum_matching_size);
}
