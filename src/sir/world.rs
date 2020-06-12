use super::person::{ Person, PersonState };
use super::virus::Virus;

#[derive(Copy, Clone, Debug)]
pub enum PopulationDistribution {
  Random,
  Grid
}

pub struct World {
  people: Vec<Person>,
  width: usize,
  height: usize,
  move_speed: usize,
  virus: Virus
}

impl World {
  /// Constructs a new World, the basic element in which we can put people and virussen.
  pub fn new(population: usize, width: usize, height: usize, virus: Virus, distribution: PopulationDistribution) -> World {
    let mut people: Vec<Person> = Vec::new();
    for index in 1..population+1 {
      match distribution {
        PopulationDistribution::Random => people.push(Person::new_random(width, height, index as usize)),
        PopulationDistribution::Grid => {
          let population = population as f32;
          let grid_width = (population).sqrt().ceil();
          let grid_height = (population / grid_width).floor();
          let x = index % (grid_width as usize) * ((width as f32 / grid_width) as usize);
          let y = index / (grid_width as usize) * ((height as f32 / grid_height) as usize);
          people.push(Person::new(x as isize, y as isize, index as usize));
        }
      }
    }
    people[0].infect(1.0);

    World {
      people,
      width,
      height,
      move_speed: 5,
      virus
    }
  }
  pub fn config(&mut self, move_speed: usize) {
    self.move_speed = move_speed;
  }
  pub fn get_width(&self) -> usize {
    self.width
  }
  pub fn get_height(&self) -> usize {
    self.height
  }
  pub fn people_mut(&mut self) -> &mut Vec<Person> {
    &mut self.people
  }
  fn infect_closeby(&mut self, max_dist: usize) {
    let max_dist: isize = max_dist as isize;
    // warning !! max number
    let mut to_infect = [false; 1_000_000];
    for i in 0..self.people.len() {
      for j in i+1..self.people.len() {
        if self.people[i].get_state() == PersonState::Susceptible && self.people[j].get_state() == PersonState::Infectious {
          let dist = self.people[i].sqr_distance(&self.people[j], self.width, self.height);
          if dist < max_dist {
            to_infect[self.people[i].get_id()] = true;
          }
        } else if self.people[j].get_state() == PersonState::Susceptible && self.people[i].get_state() == PersonState::Infectious {
          let dist = self.people[i].sqr_distance(&self.people[j], self.width, self.height);
          if dist < max_dist {
            to_infect[self.people[j].get_id()] = true;
          }
        }
      }
    }
    for person in self.people.iter_mut() {
      if to_infect[person.get_id()] {
        person.infect(self.virus.infection_rate)
      }
    }
  }
  pub fn update(&mut self) {
    for person in self.people.iter_mut() {
      person.move_random(self.move_speed, self.width, self.height);
      person.update_age(self.virus.recovery_time, self.virus.mortality_rate);
    }
    let max_dist = self.virus.distance * self.virus.distance;
    self.infect_closeby(max_dist);
  }
}

#[test]
fn update_move_speed() {
  let virus = Virus::corona();
  let mut world = World::new(1, 100, 100, virus, PopulationDistribution::Random);
  world.config(15);
  let mut person = world.people[0].clone();
  let mut max_move = 0;
  for _ in 1..1000 {
    world.update();
    let dist = person.sqr_distance(&world.people[0], 100, 100);
    if dist > max_move {
      max_move = dist;
    }
    person = world.people[0].clone();
  }
  // sqr_distance of 15 = (15^2 + 15^2)
  assert_eq!(2 * 15 * 15, max_move);
}

#[test]
fn infect_closeby_users() {
  // We make a grid with 1 user infected, next step should be 5 -> 13 -> 25
  let mut virus = Virus::corona();
  // each step, infect all neighbours
  virus.distance = 12;
  let expected_infected = [1, 5, 13, 25];
  virus.infection_rate = 1.0;
  let mut world = World::new(100, 100, 100, virus, PopulationDistribution::Grid);
  world.config(0);
  for index in 0..4 {
    let mut count = 0;
    for person in world.people_mut().iter() {
      if person.get_state() == PersonState::Infectious {
        count += 1;
      }
    }
    assert_eq!(expected_infected[index], count);
    world.update();
  }
}
