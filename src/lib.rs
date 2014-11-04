pub mod voter_input {
    use std::io;

    #[deriving(Clone)]
    pub struct DogLover {
        pub favorite_dog: uint,
        pub hated_cat: uint
    }

    #[deriving(Clone)]
    pub struct CatLover {
        pub favorite_cat: uint,
        pub hated_dog: uint
    }

    #[deriving(Clone)]
    pub enum Voter {
        DogPerson(DogLover),
        CatPerson(CatLover)
    }
    
    impl Voter {
        pub fn is_compatible(&self, other_voter: &Voter) -> bool {
            match *self {
                DogPerson(dog_lover_self) => match *other_voter {
                    DogPerson(..) => true,
                    CatPerson(cat_lover_other) => {
                        dog_lover_self.favorite_dog != cat_lover_other.hated_dog
                        && dog_lover_self.hated_cat != cat_lover_other.favorite_cat
                    }
                },
                CatPerson(cat_lover_self) => match *other_voter {
                    CatPerson(..) => true,
                    DogPerson(dog_lover_other) => {
                        dog_lover_other.favorite_dog != cat_lover_self.hated_dog
                        && dog_lover_other.hated_cat != cat_lover_self.favorite_cat
                    }
                }
            }
        }
    }
    
    #[deriving(PartialEq)]
    enum Species {
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
        
        let favorite_pet = match get_pet(split[0]) {
            Some(pet) => pet,
            None      => {
                println!("Incorrect vote! Try again.");
            
                return get_voter();
            }
        };
        
        let hated_pet = match get_pet(split[1]) {
            Some(pet) => pet,
            None      => {
                println!("Incorrect vote! Try again.");
            
                return get_voter();
            }
        };
        
        if favorite_pet.species == hated_pet.species {
            println!("Incorrect vote! Try again.");
            
            return get_voter();
        }
    
        match favorite_pet.species {
            Dog => DogPerson ( DogLover { favorite_dog: favorite_pet.number, hated_cat: hated_pet.number } ),
            Cat => CatPerson ( CatLover { favorite_cat: favorite_pet.number, hated_dog: hated_pet.number } )
        }
    }
    
    fn get_pet(code: &str) -> Option<Pet> {
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
        
        pub fn get_incidence_matrix(&self) -> &[bool] {
            self.incidence_matrix
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
        
        println!("Edge set size {}", edge_set.len());
        
        println!("Neighbour count for row {} is {}", row, range(0, columns).filter(|col| graph.has_edge(row, *col)).count());
        
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
    
        if collapsed_trace.contains(&column) {
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
            None       => matching.len(),
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
