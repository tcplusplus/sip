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
  pub fn new(population: u32, width: usize, height: usize, virus: Virus, distribution: PopulationDistribution) -> World {
    let mut people: Vec<Person> = Vec::new();
    for index in 1..population+1 {
      match distribution {
        PopulationDistribution::Random => people.push(Person::new_random(width, height)),
        PopulationDistribution::Grid => {
          let population = population as f32;
          let grid_width = (population).sqrt().ceil();
          let grid_height = (population / grid_width).floor();
          let x = index % (grid_width as u32) * ((width as f32 / grid_width) as u32);
          let y = index / (grid_width as u32) * ((height as f32 / grid_height) as u32);
          people.push(Person::new(x as isize, y as isize));
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
    for i in 0..self.people.len() {
      for j in i+1..self.people.len() {
        if self.people[i].get_state() == PersonState::Susceptible && self.people[j].get_state() == PersonState::Infectious {
          let dist = self.people[i].sqr_distance(&self.people[j], self.width, self.height);
          if dist < max_dist {
            self.people[i].infect(self.virus.infection_rate)
          }
        } else if self.people[j].get_state() == PersonState::Susceptible && self.people[i].get_state() == PersonState::Infectious {
          let dist = self.people[i].sqr_distance(&self.people[j], self.width, self.height);
          if dist < max_dist {
            self.people[j].infect(self.virus.infection_rate)
          }
        }
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

}
