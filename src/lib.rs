pub mod voter_input {
    use std::io;

    pub struct DogLover {
        favorite_dog: uint,
        hated_cat: uint
    }

    pub struct CatLover {
        favorite_cat: uint,
        hated_dog: uint
    }

    pub enum Voter {
        DogPerson(DogLover),
        CatPerson(CatLover)
    }
    
    impl Voter {
        fn is_compatible(&self, other_voter: &Voter) -> bool {
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
    pub fn fill_voter_list(mut voter_list: Vec<Voter>, voter_count: uint) -> Vec<Voter> {
        let voter_count_int: int = match voter_count.to_int() {
            Some(x) => x,
            None    => {
                fail!("Could not cast unsigned integer to signed integer!");
            }
        };

        for x in range(0i, voter_count_int) {
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
