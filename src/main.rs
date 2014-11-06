extern crate spotify;

use std::io;
use spotify::voter_input;
use spotify::bipartite_matchings::BipartiteGraph;

// todo: number of testcases can be greater one!

fn main() {
    let mut buffer = io::stdin();

    let (cat_count, dog_count, voter_count) = match voter_input::get_parameters(&mut buffer) {
        None         => fail!("Incorrect parameters"),
        Some(triple) => triple
    };
    
    let (dog_lovers, cat_lovers) = voter_input::get_voter_list(&mut buffer, cat_count, dog_count, voter_count);
    
    let graph = BipartiteGraph::from_closure(
        &mut dog_lovers.iter(), 
        &mut cat_lovers.iter(),
        |dog, cat| ! dog.is_compatible(*cat)
    );
    
    println!("{}", dog_lovers.len() + cat_lovers.len() - graph.get_max_matching_size());
}
