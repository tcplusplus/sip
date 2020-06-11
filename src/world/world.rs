use super::person::{ Person, PersonState };
use super::virus::Virus;

pub struct World {
  pub people: Vec<Person>,
  width: usize,
  height: usize,
  move_speed: usize,
  virus: Option<Virus>
}

impl World {
  pub fn new(population: u32, width: usize, height: usize) -> World {
    let mut people: Vec<Person> = Vec::new();
    for _ in 1..population+1 {
      people.push(Person::new_random(width, height))
    }
    people[0].infect(1.0);

    let world = World {
      people: people,
      width,
      height,
      move_speed: 5,
      virus: None
    };
    world
  }
  pub fn config(&mut self, move_speed: usize) {
    self.move_speed = move_speed;
  }
  pub fn set_virus(&mut self, virus: &Virus) {
    self.virus = Some(virus.clone());
  }
  pub fn get_width(&self) -> usize {
    self.width
  }
  pub fn get_height(&self) -> usize {
    self.height
  }
  fn infect_closeby(people: &mut Vec<Person>, max_dist: usize, infection_rate: f32) {
    let max_dist: isize = max_dist as isize;
    for i in 0..people.len() {
      for j in i+1..people.len() {
        if people[i].get_state() == PersonState::Susceptible && people[j].get_state() == PersonState::Infectious {
          let dist = people[i].sqr_dist(&people[j]);
          if dist < max_dist {
            people[i].infect(infection_rate)
          }
        } else if people[j].get_state() == PersonState::Susceptible && people[i].get_state() == PersonState::Infectious {
          let dist = people[i].sqr_dist(&people[j]);
          if dist < max_dist {
            people[j].infect(infection_rate)
          }
        }
      }
    }
  }
  pub fn update(&mut self) {
    for person in self.people.iter_mut() {
      person.move_random(self.move_speed, self.width, self.height);
    }
    if let Some(virus) = &self.virus {
      let max_dist = virus.distance * virus.distance;
      World::infect_closeby(&mut self.people, max_dist, virus.infection_rate);
      for person in self.people.iter_mut() {
        person.update_age(virus.recovery_time, virus.mortality_rate);
      }
    }
  }
}


