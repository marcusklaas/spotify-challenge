pub mod voter_input {
    use std::io;

    #[deriving(PartialEq)]
    pub enum Species {
        Dog,
        Cat
    }
    
    struct Pet {
        species: Species,
        number: uint
    }
    
    pub struct Voter {
        favorite_species: Species,
        dog_vote: uint,
        cat_vote: uint
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
    
    pub fn get_parameters() -> Option<(uint, uint, uint)> {
        let input: String = io::stdin().read_line().ok().expect("Failed to read line");
        let split = input.as_slice().trim().split(' ');
        let parameters: Vec<Option<uint>> = split.map(|x| from_str::<uint>(x.as_slice())).collect();
        
        match parameters.len() < 3 || parameters.iter().any(|x| x.is_none()) {
            true  => None,
            false => Some((parameters[0].unwrap(), parameters[1].unwrap(), parameters[2].unwrap()))
        }
    }
    
    // todo: only accept pets with number at most number of cats/ dogs
    pub fn get_voter_list(voter_count: uint) -> (Vec<Voter>, Vec<Voter>) {
        let mut dog_lovers = Vec::new();
        let mut cat_lovers = Vec::new();
        
        for _ in range(0, voter_count) {
            let voter = get_voter();
        
            match voter.favorite_species {
                Dog => &mut dog_lovers,
                Cat => &mut cat_lovers
            }
              .push(voter);
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
            _   => { return None; }
        };
    
        let number: uint = match from_str(code.slice_from(1)) {
            Some(x) => x,
            None    => { return None; }
        };
    
        Some( Pet { species: species, number: number } )
    }
}

pub mod bipartite_matchings {
    use std::collections::TreeSet;
    
    type Vertex = uint;
    type Edge = (Vertex, Vertex);
    type EdgeSet = TreeSet<Edge>;
    
    struct Path {
        starting_row: Vertex,
        trace: Vec<Edge>
    }
    
    impl Path {
        fn new(start: Vertex) -> Path {
            Path {
                starting_row: start,
                trace: Vec::new()
            }
        }
    
        fn get_current(&self) -> Vertex {
            let path_length = self.trace.len();
        
            match path_length > 0 {
                true  => {
                    let (row, col) = self.trace[path_length - 1];
                    
                    match self.is_odd_length() {
                        true  => col,
                        false => row
                    }
                },
                false => self.starting_row
            }
        }
        
        fn is_odd_length(&self) -> bool {
            self.trace.len() % 2 == 1
        }
        
        fn add_edge(&mut self, edge: Edge) {
            self.trace.push(edge);
        }
        
        fn remove_edge(&mut self) {
            self.trace.pop();
        }
        
        fn get_edge_set(&self) -> EdgeSet {
            self.trace.iter().map(|&x| x).collect()
        }
    }
    
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
        
        pub fn get_max_matching_size(&self) -> uint {
            let empty_matching: EdgeSet = TreeSet::new();
            
            self.max_matching_size(&empty_matching)
        }
        
        fn max_matching_size(&self, matching: &EdgeSet) -> uint {    
            match self.get_augmenting_path(matching) {
                None       => { matching.len() },
                Some(path) => {
                    let new_matching: EdgeSet = matching.symmetric_difference(&path)
                      .map(|&x| x)
                      .collect();
                
                    self.max_matching_size(&new_matching)
                }
            }
        }
        
        fn get_augmenting_path(&self, matching: &EdgeSet) -> Option<EdgeSet> {
            match self.get_unmatched_rows(matching).iter()
              .map(|&row| self.try_augmenting_path(matching, row))
              .find(|x| x.is_some()) {
                Some(x) => x,
                None    => None
            }
        }
        
        fn try_augmenting_path(&self, matching: &EdgeSet, row: Vertex) -> Option<EdgeSet> {
            let mut path = Path::new(row);
            
            self.search_path(matching, &mut path)
        }
        
        fn search_path(&self, matching: &EdgeSet, path: &mut Path) -> Option<EdgeSet> {
            let current = path.get_current();
            let is_column = path.is_odd_length();
            
            if is_column && matching.iter().map(|&(_, col)| col).find(|&x| x == current).is_none() {
                return Some(path.get_edge_set());
            }
            
            let edges = self.get_edges(current, is_column);
            let mut eligible_edges = edges.iter().filter(|&x| matching.contains(x) == is_column);
            
            for &edge in eligible_edges {
                path.add_edge(edge);
                
                match self.search_path(matching, path) {
                    Some(new_path) => { return Some(new_path); },
                    None           => { path.remove_edge(); }
                }
            }
        
            None
        }
        
        fn get_unmatched_rows(&self, matching: &EdgeSet) -> Vec<Vertex> {
            range(0, self.rows)
              .filter(|&x| !self.is_row_matched(matching, x))
              .collect()
        }
        
        fn is_row_matched(&self, matching: &EdgeSet, row: Vertex) -> bool {
            range(0, self.columns)
              .map(|col| (row, col))
              .any(|x| matching.contains(&x))
        }
        
        fn get_edges(&self, node: Vertex, is_column: bool) -> Vec<Edge> {
            match is_column {
                true  => range(0, self.rows).map(|x| (x, node)),
                false => range(0, self.columns).map(|x| (node, x))
            }
              .filter(|&(row, col)| self.incidence_matrix[row * self.columns + col])
              .collect()
        }
    }
}
