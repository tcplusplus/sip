use rand::Rng;
use rand::rngs::ThreadRng;

#[derive(Debug, PartialEq, Clone)]
pub enum PersonState {
  Susceptible,
  Infectious,
  Recovered(bool)   // add boolean dead
}

#[derive(Clone, Debug)]
pub struct Location {
  pub x: isize,
  pub y: isize
}

#[derive(Debug)]
pub struct Person {
  state: PersonState,
  infected_date: isize,
  age: isize,
  pub position: Location,
  home: Location,
  rng: ThreadRng
}

impl Person {
  pub fn new_random(max_x: usize, max_y: usize) -> Person {
    let mut rng = rand::thread_rng();
    let position = Location {
      x: rng.gen_range(0, max_x as isize),
      y: rng.gen_range(0, max_y as isize)
    };
    Person {
      state: PersonState::Susceptible,
      age: 0,
      infected_date: 0,
      home: position.clone(),
      position,
      rng
    }
  }
  pub fn get_state(&self) -> PersonState {
    self.state.clone()
  }
  pub fn infect(&mut self, infection_rate: f32) {
    let chance = self.rng.gen_range(0.0, 1.0);
    if chance <= infection_rate {
      self.state = PersonState::Infectious;
      self.infected_date = self.age;
    }
  }
  pub fn update_age(&mut self, max_infected_age: usize, mortality_rate: f32) {
    self.age += 1;
    let max_infected_age: isize = max_infected_age as isize;
    if self.state == PersonState::Infectious && self.infected_date < self.age - max_infected_age {
      let chance = self.rng.gen_range(0.0, 1.0);
      self.state = PersonState::Recovered(chance < mortality_rate);
    }
  }
  pub fn move_random(&mut self, max_speed: usize, max_x: usize, max_y: usize) {
    let mut max_speed: isize = max_speed as isize;
    if let PersonState::Recovered(is_dead) = self.state {
      if is_dead {
        max_speed = 0
      }
    }
    let x: isize = self.position.x as isize;
    let y: isize = self.position.y as isize;

    let mut new_x = x + self.rng.gen_range(-max_speed, max_speed+1);
    let mut new_y = y + self.rng.gen_range(-max_speed, max_speed+1);
    let max_x: isize = max_x as isize;
    let max_y: isize = max_y as isize;
    if new_x >= max_x {
      new_x = 0
    };
    if new_y >= max_y {
      new_y = 0
    }
    if new_x < 0 {
      new_x = max_x - 1;
    };
    if new_y  < 0 {
      new_y = max_y - 1;
    }
    self.position.x = new_x;
    self.position.y = new_y;
  }
  pub fn sqr_dist(&self, other: &Person) -> isize {
    return (self.position.x - other.position.x) * (self.position.x - other.position.x) +
           (self.position.y - other.position.y) * (self.position.y - other.position.y)
  }
}