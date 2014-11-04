pub mod voter_input {
    use std::io;

    #[deriving(Clone)]
    pub struct Voter {
        pub favorite_species: Species,
        pub dog_vote: uint,
        pub cat_vote: uint
    }
    
    impl Voter {
        pub fn is_compatible(&self, other: &Voter) -> bool {
            self.favorite_species != other.favorite_species && (
                self.dog_vote == other.dog_vote || self.cat_vote == other.cat_vote
            )
        }
        
        pub fn is_cat_person(&self) -> bool {
            self.favorite_species == Cat
        }
    }
    
    #[deriving(PartialEq, Clone)]
    pub enum Species {
        Dog,
        Cat
    }
    
    struct Pet {
        species: Species,
        number: uint
    }
    
    pub fn get_parameters() -> Result<(uint, uint, uint), String> {
        let input: String = io::stdin().read_line().ok().expect("Failed to read line");
        let split: Vec<&str> = input.as_slice().trim().split(' ').collect();
        
        if split.len() != 3 {
            return Err("Incorrect number of arguments!".to_string());
        }
        
        let cat_count: uint = match from_str(split[0].as_slice()) {
            Some(x) => x,
            None    => {
                return Err("Incorrect number of cats!".to_string());
            }
        };    
        
        let dog_count: uint = match from_str(split[1].as_slice()) {
            Some(x) => x,
            None    => {
                return Err("Incorrect number of dogs!".to_string());
            }
        };
        
        let voter_count: uint = match from_str(split[2].as_slice()) {
            Some(x) => x,
            None    => {
                return Err("Incorrect number of voters!".to_string());
            }
        };
        
        let result: (uint, uint, uint) = (cat_count, dog_count, voter_count);
        
        Ok(result)
    }
    
    // todo: only accept pets with number at most number of cats/ dogs
    pub fn fill_voter_list(voter_list: &mut [Voter], voter_count: uint) {
        for x in range(0, voter_count) {
            voter_list[x] = get_voter();
        }
    }

    fn get_voter() -> Voter {
        let input: String = io::stdin().read_line().ok().expect("Failed to read line");
        let split: Vec<&str> = input.as_slice().trim().split(' ').collect();
        
        if split.len() != 2 {
            println!("Incorrect vote! Try again.");
            
            return get_voter();
        }
        
        let favorite_pet = match read_pet(split[0]) {
            Some(pet) => pet,
            None      => {
                println!("Incorrect vote! Try again.");
            
                return get_voter();
            }
        };
        
        let least_favorite_pet = match read_pet(split[1]) {
            Some(pet) => pet,
            None      => {
                println!("Incorrect vote! Try again.");
            
                return get_voter();
            }
        };
        
        if favorite_pet.species == least_favorite_pet.species {
            println!("Incorrect vote! Try again.");
            
            return get_voter();
        }
    
        Voter {
            favorite_species: favorite_pet.species,
            cat_vote: match favorite_pet.species {
                Cat => favorite_pet.number,
                Dog => least_favorite_pet.number
            },
            dog_vote: match favorite_pet.species {
                Dog => favorite_pet.number,
                Cat => least_favorite_pet.number
            }
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
    
    pub struct BipartiteGraph<'a> {
        rows: uint,
        columns: uint,
        incidence_matrix: &'a mut [bool]
    }
    
    impl<'a> BipartiteGraph<'a> {
        pub fn new<'a>(rows: uint, columns: uint, vec: &'a mut Vec<bool>) -> BipartiteGraph<'a> {
            vec.clear();
            
            for _ in range(0, rows * columns) {
                vec.push(false);
            }
        
            BipartiteGraph {
                rows: rows,
                columns: columns,
                incidence_matrix: vec.as_mut_slice()
            }
        }
        
        pub fn from_closure<R, C, T: Clone + Iterator<R>, U: Clone + Iterator<C>>(rows: &T, columns: &U, closure: |&R, &C| -> bool, vec: &'a mut Vec<bool>) -> BipartiteGraph<'a> {
            vec.clear();
            
            let mut row_iterator = rows.clone();
            let mut row_count = 0u;
            
            for row in row_iterator {
                let mut column_iterator = columns.clone();
                
                for col in column_iterator {
                    vec.push(closure(&row, &col));
                }
            
                row_count += 1;
            }
            
            let column_count = columns.clone().count();
            
            BipartiteGraph {
                rows: row_count,
                columns: column_count,
                incidence_matrix: vec.as_mut_slice()
            }
        }
        
        pub fn set_edge(&mut self, i: uint, j: uint, edginess: bool) {
            self.incidence_matrix[self.columns * i + j] = edginess;
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
    
    fn collapse_trace(trace: &Vec<Edge>) -> TreeSet<uint> {
        let mut result: TreeSet<uint> = TreeSet::new();
        
        for (row, col) in trace.iter().map(|&x| x) {
            result.insert(row);
            result.insert(col);
        }
        
        result
    }
    
    fn trace_to_set(trace: &Vec<Edge>) -> EdgeSet {
        let mut set = TreeSet::new();
        
        for edge in trace.iter() {
            set.insert(*edge);
        }
        
        set
    }
    
    fn augment_row(graph: &BipartiteGraph, matching: &EdgeSet, trace: &mut Vec<Edge>, row: uint) -> Option<EdgeSet> {
        let collapsed_trace = collapse_trace(trace);
        
        let (_, columns) = graph.get_dimensions();
        
        let unvisited_neighbours = range(0, columns)
                                .filter(|col| graph.has_edge(row, *col))
                                .filter(|col| !collapsed_trace.contains(col));
                                
        let edge_set: TreeSet<Edge> = unvisited_neighbours.map(|col| (row, col)).collect();
        
        let mut unmatched_edges = edge_set.difference(matching);
        
        for &edge in unmatched_edges {
            let (_, col) = edge;
            
            println!("Unmatched edge {}", edge);
            
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
        let collapsed_trace = collapse_trace(trace);
        
        let matched_columns: TreeSet<uint> = matching.iter().map(|&(row, col)| col).collect();
    
        if ! matched_columns.contains(&column) {
            println!("Found augmenting path!");
        
            return Some(trace_to_set(trace));
        }
        
        let (rows, _) = graph.get_dimensions();
        
        let visited_neighbours = range(0, rows)
                                .filter(|row| graph.has_edge(*row, column))
                                .filter(|row| collapsed_trace.contains(row));
                                
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
        let unmatched_rows = get_unmatched_rows(graph, matching);
        let mut trace: Vec<Edge> = Vec::new();
        
        println!("Unmatched rows: {}", unmatched_rows.as_slice());
        
        for row in unmatched_rows.iter() {
            match augment_row(graph, matching, &mut trace, *row) {
                Some(path) => {
                    return Some(path);
                },
                None => {
                    trace.clear();
                }
            }
        }
      
        None
    }
    
    fn max_matching_size(graph: &BipartiteGraph, matching: &EdgeSet) -> uint {    
        match get_augmenting_path(graph, matching) {
            None       => {
                println!("Maximum matching: {}", matching);
            
                matching.len()
            },
            Some(path) => {
                let mut new_matching: EdgeSet = TreeSet::new();
                
                for x in matching.symmetric_difference(&path) {
                    new_matching.insert(x.clone());
                }
            
                max_matching_size(graph, &new_matching)
            }
        }
    }
    
    pub fn get_max_matching_size(graph: &BipartiteGraph) -> uint {
        let empty_matching: EdgeSet = TreeSet::new();
        
        max_matching_size(graph, &empty_matching)
    }
}
