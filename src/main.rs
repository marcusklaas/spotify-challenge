extern crate spotify;

use std::io::stdio::{stdin, StdinReader};
use spotify::voter_input::get_voter_list;
use spotify::bipartite_matchings::BipartiteGraph;

fn main() {
    let mut buffer = stdin();
    let input = buffer.read_line().ok().expect("Could not read line!");
    let testcase_count = from_str::<uint>(input.as_slice().trim()).expect("Invalid testcase count!");
    
    for _ in range(0, testcase_count) {
        println!("{}", run_test_case(&mut buffer));
    }
}

fn run_test_case(buffer: &mut StdinReader) -> uint {
    let (dog_lovers, cat_lovers) = get_voter_list(buffer);
    
    let graph = BipartiteGraph::from_closure(
        dog_lovers.as_slice(), 
        cat_lovers.as_slice(),
        |dog, cat| ! dog.is_compatible(cat)
    );
    
    dog_lovers.len() + cat_lovers.len() - graph.get_max_matching_size()
}
