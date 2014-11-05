pub mod voter_input {
    use std::io;

    #[deriving(PartialEq, Clone)]
    pub enum Species {
        Dog,
        Cat
    }
    
    struct Pet {
        species: Species,
        number: uint
    }

    #[deriving(Clone)]
    pub struct Voter {
        pub favorite_species: Species,
        pub dog_vote: uint,
        pub cat_vote: uint
    }
    
    impl Voter {
        pub fn is_compatible(&self, other: &Voter) -> bool {
            self.favorite_species == other.favorite_species || (
                self.dog_vote != other.dog_vote && self.cat_vote != other.cat_vote
            )
        }
        
        pub fn is_cat_person(&self) -> bool {
            self.favorite_species == Cat
        }
    }
    
    pub fn get_parameters() -> Result<(uint, uint, uint), String> {
        let input: String = io::stdin().read_line().ok().expect("Failed to read line");
        let split: Vec<&str> = input.as_slice().trim().split(' ').collect();
        
        if split.len() != 3 {
            return Err("Incorrect number of arguments!".to_string());
        }
        
        let parameters: Vec<Option<uint>> = split.iter().map(|&x| from_str::<uint>(x.as_slice())).collect();
        
        if parameters.iter().any(|&x| x == None) {
            return Err("Input were not valid non-negative numbers!".to_string());
        }
        
        Ok((parameters[0].unwrap(), parameters[1].unwrap(), parameters[2].unwrap()))
    }
    
    // todo: only accept pets with number at most number of cats/ dogs
    pub fn get_voter_list(voter_count: uint) -> (Vec<Voter>, Vec<Voter>) {
        let mut dog_lovers = Vec::new();
        let mut cat_lovers = Vec::new();
        
        for _ in range(0, voter_count) {
            let voter = get_voter();
        
            let list = match voter.favorite_species {
                Dog => &mut dog_lovers,
                Cat => &mut cat_lovers
            };
        
            list.push(voter);
        }
        
        (cat_lovers, dog_lovers)
    }

    fn get_voter() -> Voter {
        match read_voter() {
            Some(voter) => voter,
            None        => {
                println!("Incorrect vote! Try again.");
                get_voter()
            }
        }
    }
    
    fn read_voter() -> Option<Voter> {
        let input: String = io::stdin().read_line().ok().expect("Failed to read line");
        let split = input.as_slice().trim().split(' ');        
        let pets: Vec<Option<Pet>> = split.map(read_pet).collect();
        
        if pets.len() != 2 || pets.iter().any(|x| x.is_none()) {
            return None;
        }
        
        let real_pets: Vec<Pet> = pets.iter().map(|x| x.unwrap()).collect();
        let favorite_species = real_pets[0].species;
        
        match favorite_species == real_pets[1].species {
            false => Some(Voter{
                favorite_species: favorite_species,
                cat_vote: match favorite_species {
                    Cat => real_pets[0].number,
                    Dog => real_pets[1].number
                },
                dog_vote: match favorite_species {
                    Dog => real_pets[0].number,
                    Cat => real_pets[1].number
                }
            }),
            true => None
        }
    }
    
    fn read_pet(code: &str) -> Option<Pet> {
        if code.len() < 2 {
            return None;
        }
        
        let species: Species = match code.char_at(0) {
            'C' => Cat,
            'D' => Dog,
            _   => {
                return None;
            }
        };
    
        let number: uint = match from_str(code.slice_from(1)) {
            Some(x) => x,
            None    => {
                return None;
            }
        };
    
        Some( Pet { species: species, number: number } )
    }
}

pub mod bipartite_matchings {
    use std::collections::TreeSet;
    
    pub struct BipartiteGraph {
        rows: uint,
        columns: uint,
        incidence_matrix: Vec<bool>
    }
    
    impl BipartiteGraph {
        pub fn from_closure<R, C, T: Iterator<R>, U: Iterator<C>>(rows: &mut T, columns: &mut U, closure: |&R, &C| -> bool) -> BipartiteGraph {
            let mut vec: Vec<bool> = Vec::new();
            let column_set: Vec<C> = columns.collect();
            
            for row in *rows {
                for col in column_set.iter() {
                    vec.push(closure(&row, col));
                }
            }
            
            BipartiteGraph {
                rows: vec.len()/ column_set.len(),
                columns: column_set.len(),
                incidence_matrix: vec
            }
        }
        
        fn has_edge(&self, i: uint, j: uint) -> bool {
            self.incidence_matrix[self.columns * i + j]
        }
        
        pub fn get_dimensions(&self) -> (uint, uint) {
            (self.rows, self.columns)
        }
    }
    
    trait SymmetricDifferenceSet {
        fn new_symmetric_difference(&self, other: &Self) -> Self;
    }
    
    type Edge = (uint, uint);
    type EdgeSet = TreeSet<Edge>;
    
    fn get_unmatched_rows(graph: &BipartiteGraph, matching: &EdgeSet) -> Vec<uint> {
        let (rows, _) = graph.get_dimensions();
        
        range(0, rows).filter(|&x| !is_row_matched(graph, matching, x)).collect()
    }
    
    fn is_row_matched(graph: &BipartiteGraph, matching: &EdgeSet, row: uint) -> bool {
        let (_, columns) = graph.get_dimensions();
        
        range(0, columns).map(|col| (row, col)).any(|x| matching.contains(&x))
    }
    
    fn collapse_trace(trace: &Vec<Edge>, closure: |&(uint, uint)| -> uint) -> TreeSet<uint> {
        trace.iter().map(closure).collect()
    }
    
    fn trace_to_set(trace: &Vec<Edge>) -> EdgeSet {
        trace.iter().map(|&x| x).collect()
    }
    
    fn augment_row(graph: &BipartiteGraph, matching: &EdgeSet, trace: &mut Vec<Edge>, row: uint) -> Option<EdgeSet> {
        let visited_columns = collapse_trace(trace, |&(_, y)| y);        
        let (_, columns) = graph.get_dimensions();
        
        let unvisited_neighbours = range(0, columns)
            .filter(|col| graph.has_edge(row, *col) && !visited_columns.contains(col));
                                
        let edge_set: TreeSet<Edge> = unvisited_neighbours.map(|col| (row, col)).collect();
        
        let mut unmatched_edges = edge_set.difference(matching);
        
        for &edge in unmatched_edges {
            let (_, col) = edge;
            
            trace.push(edge);
            
            match augment_column(graph, matching, trace, col) {
                Some(path) => {
                    return Some(path);
                },
                None => {
                    trace.pop();
                }
            }
        }
    
        None
    }
    
    fn augment_column(graph: &BipartiteGraph, matching: &EdgeSet, trace: &mut Vec<Edge>, column: uint) -> Option<EdgeSet> {
        let visited_rows = collapse_trace(trace, |&(x, _)| x);
        let matched_columns: TreeSet<uint> = matching.iter().map(|&(_, col)| col).collect();
    
        if ! matched_columns.contains(&column) {
            return Some(trace_to_set(trace));
        }
        
        let (rows, _) = graph.get_dimensions();
        
        let visited_neighbours = range(0, rows)
            .filter(|row| graph.has_edge(*row, column) && ! visited_rows.contains(row));
                                
        let edge_set: TreeSet<Edge> = visited_neighbours.map(|row| (row, column)).collect();
        
        let mut matched_edges = edge_set.intersection(matching);
        
        for &edge in matched_edges {
            let (row, _) = edge;
            
            trace.push(edge);
            
            match augment_row(graph, matching, trace, row) {
                Some(path) => {
                    return Some(path);
                },
                None => {
                    trace.pop();
                }
            }
        }
    
        None
    }
    
    fn get_augmenting_path(graph: &BipartiteGraph, matching: &EdgeSet) -> Option<EdgeSet> {
        match get_unmatched_rows(graph, matching).iter()
          .map(|&row| try_augmenting_path(graph, matching, row))
          .find(|x| x.is_some()) {
            Some(x) => x,
            None    => None
        }
    }
    
    fn try_augmenting_path(graph: &BipartiteGraph, matching: &EdgeSet, row: uint) -> Option<EdgeSet> {
        let mut trace: Vec<Edge> = Vec::new();
        
        augment_row(graph, matching, &mut trace, row)
    }
    
    fn max_matching_size(graph: &BipartiteGraph, matching: &EdgeSet) -> uint {    
        match get_augmenting_path(graph, matching) {
            None       => {
                matching.len()
            },
            Some(path) => {
                let new_matching: EdgeSet = matching.symmetric_difference(&path)
                        .map(|x| x.clone())
                        .collect();
            
                max_matching_size(graph, &new_matching)
            }
        }
    }
    
    pub fn get_max_matching_size(graph: &BipartiteGraph) -> uint {
        let empty_matching: EdgeSet = TreeSet::new();
        
        max_matching_size(graph, &empty_matching)
    }
}
