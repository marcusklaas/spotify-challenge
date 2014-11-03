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
    use std::fmt;
    
    pub struct BipartiteGraph<'a> {
        m: uint,
        pub incidence_matrix: &'a mut [bool]
    }
    
    impl<'a> BipartiteGraph<'a> {
        pub fn new<'a>(m: uint, n: uint, vec: &'a mut Vec<bool>) -> BipartiteGraph<'a> {
            vec.clear();
            
            for _ in range(0, m * n) {
                vec.push(false);
            }
        
            BipartiteGraph {
                m: m,
                incidence_matrix: vec.as_mut_slice()
            }
        }
        
        pub fn set_edge(&mut self, i: uint, j: uint, edginess: bool) {
            self.incidence_matrix[self.m * i + j] = edginess;
        }
        
        fn get_edge(&self, i: uint, j: uint) -> bool {
            self.incidence_matrix[self.m * i + j]
        }
    }
    
//    impl<'a> fmt::Show for BipartiteGraph<'a> {
//        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//            // The `f` value implements the `Writer` trait, which is what the
//            // write! macro is expecting. Note that this formatting ignores the
//            // various flags provided to format strings.
//            
//            for row in self.incidence_matrix.as_slice().chunks(self.m).collect() {
//                writeln!(f, "{}", row);
//            }
//        }
//    }
    
    trait SymmetricDifferenceSet {
        fn new_symmetric_difference(&self, other: &Self) -> Self;
    }
    
    #[deriving(Eq, Ord, Clone)]
    pub struct Edge(uint, uint);
    
    impl PartialEq for Edge {
        fn eq(&self, other: &Edge) -> bool {
            let Edge(self_first, self_last) = *self;
            let Edge(other_first, other_last) = *other;
            
            self_first == other_first && self_last == other_last 
            || self_first == other_last && self_last == other_first
        }
    }
    
    impl PartialOrd for Edge {
        fn lt(&self, other: &Edge) -> bool {
            let Edge(self_first, self_last) = *self;
            let Edge(other_first, other_last) = *other;
            
            self_first < other_first
            || self_first == other_first && self_last < other_last
        }
        
        fn partial_cmp(&self, other: &Edge) -> Option<Ordering> {
            match self.lt(other) {
                true  => Some(Less),
                false => match self.eq(other) {
                    true  => Some(Equal),
                    false => Some(Greater)
                }
            }
        }
    }
    
    pub type EdgeSet = TreeSet<Edge>;
    
//    impl<T: Ord + Clone> SymmetricDifferenceSet for TreeSet<T> {
//        fn new_symmetric_difference(&self, other: &TreeSet<T>) -> TreeSet<T> {
//            let mut set = TreeSet::new();
            
//            for x in self.symmetric_difference(other) {
//                set.insert(x.clone());
//            }
        
//            set
//        }
//    }
    
//    pub fn get_augmenting_path(graph: BipartiteGraph, matching: EdgeSet) -> Option<EdgeSet> {
//        None
//    }
    
//    pub fn get_max_matching_size(graph: BipartiteGraph, matching: EdgeSet) -> int {
//        match get_augmenting_path(graph, matching) {
//            None       => matching.len(),
//            Some(path) => get_max_matching_size(graph, matching.new_symmetric_difference(path))
//        }
//    }
}
