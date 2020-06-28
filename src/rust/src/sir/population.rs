use super::person::{Person, PersonState};
use super::virus::Virus;
use std::iter::Flatten;
use std::slice::{Iter, IterMut};

pub struct Population {
  people: Vec<Vec<Vec<Person>>>,
  grid_width: f32,
  grid_height: f32,
}

impl Population {
  pub fn new(world_width: f32, world_height: f32, num_grid_width: usize, num_grid_height: usize) -> Population {
    let mut people: Vec<Vec<Vec<Person>>> = Vec::with_capacity(num_grid_width);
    for i in 0..num_grid_width {
      people.push(Vec::with_capacity(num_grid_height));
      for _ in 0..num_grid_height {
        people[i].push(Vec::new());
      }
    }
    Population {
      people,
      grid_width: world_width / num_grid_width as f32,
      grid_height: world_height / num_grid_height as f32
    }
  }
  fn get_indexes(&self, x: f32, y: f32) -> (usize, usize) {
    let mut grid_x = (x / self.grid_width).floor() as usize;
    let mut grid_y = (y / self.grid_height).floor() as usize;
    grid_x = grid_x % self.people.len();
    grid_y = grid_y % self.people[0].len();
    (grid_x, grid_y)
  }
  pub fn add(&mut self, person: Person) {
    let (i, j) = self.get_indexes(person.position.x, person.position.y);
    self.people[i][j].push(person);
  }
  pub fn iter(&self) -> std::iter::Flatten<Flatten<Iter<'_, Vec<Vec<Person>>>>> {
    self.people.iter().flatten().flatten()
  }
  fn iter_mut(&mut self) -> std::iter::Flatten<Flatten<IterMut<'_, Vec<Vec<Person>>>>> {
    self.people.iter_mut().flatten().flatten()
  }
  pub fn update_positions(&mut self, move_speed: f32) {
    let world_width = self.grid_width * self.people.len() as f32;
    let world_height = self.grid_width * self.people[0].len() as f32;
    let current_age = self.iter().next().unwrap().age;
    println!("Current Age is {}", current_age);
    for row in 0..self.people.len() {
      for col in 0..self.people[row].len() {
        let mut index = 0;
        while index < self.people[row][col].len() {
          println!("index {} {}", index, self.people[row][col].len());
          let mut removed_item = false;
          if self.people[row][col][index].age == current_age {
            self.people[row][col][index].move_random(move_speed, world_width, world_height);
            self.people[row][col][index].update_age();
            let new_position = &self.people[row][col][index].position;
            let (new_x, new_y) = self.get_indexes(new_position.x, new_position.y);

            if new_x != row || new_y != col {
              let person = self.people[row][col].remove(index);
              removed_item = true;
              println!("move to new box {} {} -> {} {}", row, col, new_x, new_y);
              self.people[new_x][new_y].push(person);
            }
          }
          if !removed_item {
            index += 1;
          }
        }
      }
    }
  }
  fn people_from(&self, box_x: isize, box_y: isize) -> Iter<Person> {
    let mut box_x = box_x;
    let mut box_y = box_y;
    if box_x < 0 {
      box_x = self.people.len() as isize;
    }
    if box_x == self.people.len() as isize {
      box_x = 0;
    }
    if box_y < 0 {
      box_y = self.people[0].len() as isize;
    }
    if box_y == self.people[0].len() as isize {
      box_y = 0;
    }
    println!("Get people from {} {}", box_x, box_y);
    self.people[box_x as usize][box_y as usize].iter()
  }
  fn infections_for_people_within_box(&self, box_x: usize, box_y: usize) -> Vec<(usize, Virus)> {
    let mut infections: Vec<(usize, Virus)> = Vec::new();
    let world_width = self.grid_width * self.people.len() as f32;
    let world_height = self.grid_width * self.people[0].len() as f32;
    for person1 in self.people[box_x][box_y].iter() {
      let box_x = box_x as isize;
      let box_y = box_y as isize;
      if let PersonState::Infectious(virus) = person1.get_state() {
        for x in box_x-1..box_x+2 {
          for y in box_y-1..box_y+2 {
            for person2 in self.people_from(x, y) {
              let dist = person1.sqr_distance(&person2, world_width, world_height);
              println!("Check distances {} {}", dist, virus.distance * virus.distance);
              if dist < virus.distance * virus.distance {
                infections.push((person2.get_id(), virus.clone()));
              }
            }
          }
        }
      }
    }
    infections
  }
  pub fn infect_closeby(&mut self) {
    let mut to_infect: Vec<Option<Virus>> = Vec::new();
    for _ in self.iter() {
      to_infect.push(None);
    }
    for box_x in 0..self.people.len() {
      for box_y in 0..self.people[0].len() {
        for infection in self.infections_for_people_within_box(box_x, box_y) {
          to_infect[infection.0] = Some(infection.1);
        }
      }
    }
    for person in self.iter_mut() {
      if let Some(virus) = &to_infect[person.get_id()] {
        person.infect(virus.clone())
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn correct_amount_of_boxes_is_made() {
    let population = Population::new(100.0, 100.0, 10, 5);
    assert_eq!(population.people.len(), 10);
    for i in 0..10 {
      assert_eq!(population.people[i].len(), 5);
    }
  }

  #[test]
  fn updating_positions_everybody_in_different_location() {
    let mut population = Population::new(100.0, 100.0, 10, 5);
    for index in 0..100 {
      population.add(Person::new(index as f32, index as f32, index));
    }
    population.update_positions(10.0);
    for person in population.iter() {
      assert!(person.position.x != person.get_id() as f32 && person.position.y != person.get_id() as f32);
    }
  }

  #[test]
  fn updating_positions_everybody_in_correct_boxes() {
    let mut population = Population::new(100.0, 100.0, 10, 5);
    for index in 0..100 {
      population.add(Person::new(index as f32, index as f32, index));
    }
    population.update_positions(10.0);
    for row in 0..population.people.len() {
      for col in 0..population.people[row].len() {
        for index in 0..population.people[row][col].len() {
          let person = &population.people[row][col][index];
          let (x, y) = population.get_indexes(person.position.x, person.position.y);
          assert_eq!(x, row);
          assert_eq!(y, col);
        }
      }
    }
  }

  #[test]
  fn people_are_added_to_correct_box() {
    let mut population = Population::new(100.0, 100.0, 10, 10);
    let person = Person::new(12.0, 23.0, 1);
    population.add(person);
    assert_eq!(population.people[1][2].len(), 1);
  }

  #[test]
  fn iterator_through_all_persons() {
    let mut population = Population::new(100.0, 100.0, 10, 10);
    population.add(Person::new(12.0, 23.0, 1));
    population.add(Person::new(32.0, 13.0, 0));
    let mut found = [false, false];
    for person in population.iter() {
      found[person.get_id()] = true;
    }
    assert!(found[0]);
    assert!(found[1]);
  }
}