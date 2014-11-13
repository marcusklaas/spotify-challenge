extern crate spotify;

use std::io;
use std::io::{Reader, BufferedReader};
use spotify::voter_input;
use spotify::bipartite_matchings::BipartiteGraph;

fn main() {
    let mut buffer = io::stdin();
    let input = buffer.read_line().ok().expect("Could not read line!");
    let testcase_count = from_str::<uint>(input.as_slice().trim()).expect("Invalid testcase count!");
    
    for _ in range(0, testcase_count) {
        run_test_case(&mut buffer);
    }
}

fn run_test_case<R: Reader>(buffer: &mut BufferedReader<R>) {
    let (cat_count, dog_count, voter_count) = match voter_input::get_parameters(buffer) {
        None         => panic!("Incorrect parameters"),
        Some(triple) => triple
    };
    
    let (dog_lovers, cat_lovers) = voter_input::get_voter_list(buffer, cat_count, dog_count, voter_count);
    
    let graph = BipartiteGraph::from_closure(
        &mut dog_lovers.iter(), 
        &mut cat_lovers.iter(),
        |dog, cat| ! dog.is_compatible(*cat)
    );
    
    println!("{}", dog_lovers.len() + cat_lovers.len() - graph.get_max_matching_size());
}
