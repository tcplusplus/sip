#[derive(Debug)]
pub enum PersonState {
  Susceptible,
  Infectious,
  Recovered   // add boolean dead
}

#[derive(Debug)]
pub struct Location {
  x: i32,
  y: i32
}

#[derive(Debug)]
pub struct Person {
  state: PersonState,
  position: Location
}

impl Person {
  pub fn new_random() -> Person {
    let position = Location {
      x: 0,
      y: 0
    };
    Person {
      state: PersonState::Susceptible,
      position
    }
  }
}