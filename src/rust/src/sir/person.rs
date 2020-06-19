use rand::rngs::ThreadRng;
use rand::Rng;

#[derive(Debug, PartialEq, Clone)]
pub enum PersonState {
    Susceptible,
    Infectious,
    Recovered(bool),
}

#[derive(Clone, Debug)]
pub struct Location {
    pub x: isize,
    pub y: isize,
}

#[derive(Debug, Clone)]
pub struct Person {
    id: usize,
    state: PersonState,
    infected_date: isize,
    age: isize,
    pub position: Location,
    home: Location,
    rng: ThreadRng,
}

impl Person {
    pub fn new_random(max_x: usize, max_y: usize, id: usize) -> Person {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0, max_x as isize);
        let y = rng.gen_range(0, max_y as isize);
        Person::new(x, y, id)
    }
    pub fn new(x: isize, y: isize, id: usize) -> Person {
        let rng = rand::thread_rng();
        let position = Location { x, y };
        Person {
            id,
            state: PersonState::Susceptible,
            age: 0,
            infected_date: 0,
            home: position.clone(),
            position,
            rng,
        }
    }
    pub fn get_state(&self) -> PersonState {
        self.state.clone()
    }
    pub fn get_id(&self) -> usize {
        self.id
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
        if self.state == PersonState::Infectious && self.infected_date < self.age - max_infected_age
        {
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

        let mut new_x = x + self.rng.gen_range(-max_speed, max_speed + 1);
        let mut new_y = y + self.rng.gen_range(-max_speed, max_speed + 1);
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
        if new_y < 0 {
            new_y = max_y - 1;
        }
        self.position.x = new_x;
        self.position.y = new_y;
    }
    fn min_diff(x1: isize, x2: isize, width: isize) -> isize {
        let diff_1 = (x1 - x2).abs();
        let diff_2 = (diff_1 - width).abs();
        std::cmp::min(diff_1, diff_2)
    }
    // We need the world to know its size because the world is circular
    pub fn sqr_distance(&self, other: &Person, world_width: usize, world_height: usize) -> isize {
        let diff_x = Person::min_diff(self.position.x, other.position.x, world_width as isize);
        let diff_y = Person::min_diff(self.position.y, other.position.y, world_height as isize);
        diff_x * diff_x + diff_y * diff_y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dead_people_dont_move() {
        let rng = rand::thread_rng();
        let position = Location { x: 10, y: 10 };
        let mut person = Person {
            id: 1,
            state: PersonState::Recovered(true),
            age: 0,
            infected_date: 0,
            home: position.clone(),
            position,
            rng,
        };
        for _ in 0..10 {
            person.move_random(10, 100, 100);
            assert_eq!(person.position.x, 10);
            assert_eq!(person.position.y, 10);
        }
    }
}