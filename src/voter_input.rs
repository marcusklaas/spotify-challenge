use std::io::{Reader, BufferedReader};

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

pub fn get_parameters<R: Reader>(buffer: &mut BufferedReader<R>) -> Option<(uint, uint, uint)> {
    let input = buffer.read_line().ok().expect("Failed to read line");
    let split = input.as_slice().trim().split(' ');
    let parameters: Vec<Option<uint>> = split.map(|x| from_str::<uint>(x.as_slice())).collect();
    
    if parameters.len() < 3 || parameters.iter().any(|x| x.is_none()) {
        return None;
    }
    
    let cat_count = parameters[0].unwrap();
    let dog_count = parameters[1].unwrap();
    let voter_count = parameters[2].unwrap();
    
    match cat_count < 1 || dog_count > 100 || voter_count > 500 {
        true  => None,
        false => Some((cat_count, dog_count, voter_count))
    }
}

pub fn get_voter_list<R: Reader>(buffer: &mut BufferedReader<R>, cat_count: uint, dog_count: uint, voter_count: uint) -> (Vec<Voter>, Vec<Voter>) {
    Vec::from_fn(voter_count, |_| get_voter(buffer, cat_count, dog_count)).partition(|v| v.is_cat_person())
}

fn get_voter<R: Reader>(buffer: &mut BufferedReader<R>, cat_count: uint, dog_count: uint) -> Voter {
    match read_voter(buffer, cat_count, dog_count) {
        Some(voter) => voter,
        None        => {
            println!("Incorrect vote! Try again.");
            get_voter(buffer, cat_count, dog_count)
        }
    }
}

fn read_voter<R: Reader>(buffer: &mut BufferedReader<R>, cat_count: uint, dog_count: uint) -> Option<Voter> {
    let input = buffer.read_line().ok().expect("Failed to read line");
    let split = input.as_slice().trim().split(' ');        
    let pets: Vec<Option<Pet>> = split.map(|x| read_pet(x, cat_count, dog_count)).collect();
    
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

fn read_pet(code: &str, cat_count: uint, dog_count: uint) -> Option<Pet> {
    if code.len() < 2 {
        return None;
    }
    
    let species: Species = match code.char_at(0) {
        'C' => Cat,
        'D' => Dog,
        _   => { return None; }
    };

    let number = match from_str::<uint>(code.slice_from(1)) {
        Some(x) if x > 0 => x,
        _                => { return None; }
    };
    
    if species == Cat && number > cat_count || species == Dog && number > dog_count {
        return None;
    }

    Some( Pet { species: species, number: number } )
}
