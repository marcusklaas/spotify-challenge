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
    pub fn get_voter_list(voter_count: uint) -> Vec<Voter> {
        let mut voter_list = Vec::new();
        
        for _ in range(0, voter_count) {
            voter_list.push(get_voter());
        }
        
        voter_list
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
    
    pub struct BipartiteGraph {
        rows: uint,
        columns: uint,
        incidence_matrix: Vec<bool>
    }
    
    impl BipartiteGraph {
        pub fn new(rows: uint, columns: uint) -> BipartiteGraph {
            BipartiteGraph {
                rows: rows,
                columns: columns,
                incidence_matrix: Vec::from_elem(rows * columns, false)
            }
        }
        
        pub fn from_closure<R, C, T: Iterator<R>, U: Iterator<C>>(rows: &mut T, columns: &mut U, closure: |&R, &C| -> bool) -> BipartiteGraph {
            let mut vec: Vec<bool> = Vec::new();
            
            let mut row_count = 0u;
            
            for row in *rows {                
                for col in *columns {
                    vec.push(closure(&row, &col));
                }
            
                row_count += 1;
            }
            
            BipartiteGraph {
                rows: row_count,
                columns: 5,
                incidence_matrix: vec
            }
        }
        
        pub fn set_edge(&mut self, i: uint, j: uint, edginess: bool) {
            *self.incidence_matrix.get_mut(self.columns * i + j) = edginess;
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
        let mut set = TreeSet::new();
        
        for edge in trace.iter() {
            set.insert(*edge);
        }
        
        set
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
        let matched_columns: TreeSet<uint> = matching.iter().map(|&(row, col)| col).collect();
    
        if ! matched_columns.contains(&column) {
            return Some(trace_to_set(trace));
        }
        
        let (rows, _) = graph.get_dimensions();
        
        let visited_neighbours = range(0, rows)
                                .filter(|row| graph.has_edge(*row, column))
                                .filter(|row| ! visited_rows.contains(row));
                                
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
