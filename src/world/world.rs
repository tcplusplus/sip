use super::person::Person;

pub struct World {
  pub people: Vec<Person>
}

impl World {
  pub fn new(population: u32) -> World {
    let mut people: Vec<Person> = Vec::new();
    for _ in 1..population {
      people.push(Person::new_random())
    }

    let world = World {
      people: people
    };
    world
  }
}


