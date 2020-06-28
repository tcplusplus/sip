use super::virus::Virus;
use rand::rngs::ThreadRng;
use rand::Rng;

#[derive(Debug, PartialEq, Clone)]
pub enum PersonState {
    Susceptible,
    Infectious(Virus),
    Recovered(bool),
}

#[derive(Clone, Debug)]
pub struct Location {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone)]
pub struct Person {
    id: usize,
    state: PersonState,
    infected_date: usize,
    pub age: usize,
    pub position: Location,
    home: Location,
    rng: ThreadRng,
}

impl Person {
    pub fn new_random(max_x: f32, max_y: f32, id: usize) -> Person {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0.0, max_x as f32);
        let y = rng.gen_range(0.0, max_y as f32);
        Person::new(x, y, id)
    }
    pub fn new(x: f32, y: f32, id: usize) -> Person {
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
    pub fn infect(&mut self, virus: Virus) {
        if self.state == PersonState::Susceptible {
            let chance = self.rng.gen_range(0.0, 1.0);
            if chance <= virus.infection_rate {
                self.state = PersonState::Infectious(virus);
                self.infected_date = self.age;
            }
        }
    }
    pub fn update_age(&mut self) {
        self.age += 1;
        if let PersonState::Infectious(virus) = &self.state {
            if self.infected_date + virus.recovery_time < self.age
            {
                let chance = self.rng.gen_range(0.0, 1.0);
                self.state = PersonState::Recovered(chance < virus.mortality_rate);
            }
        }
    }
    pub fn move_random(&mut self, max_speed: f32, max_x: f32, max_y: f32) {
        if let PersonState::Recovered(is_dead) = self.state {
            if is_dead {
                return ()
            }
        }
        let x = self.position.x;
        let y = self.position.y;
        let delta = 0.0000001;
        // delta to make sure that if max_speed is 0, this wont panic
        let mut new_x = x + self.rng.gen_range(-max_speed - delta, max_speed + delta);
        let mut new_y = y + self.rng.gen_range(-max_speed - delta, max_speed + delta);
        if new_x >= max_x {
            new_x = 0.0;
        };
        if new_y >= max_y {
            new_y = 0.0;
        }
        if new_x < 0.0 {
            new_x = max_x - 1.0;
        };
        if new_y < 0.0 {
            new_y = max_y - 1.0;
        }
        self.position.x = new_x;
        self.position.y = new_y;
    }
    fn min_diff(x1: f32, x2: f32, width: f32) -> f32 {
        let diff_1 = (x1 - x2).abs();
        let diff_2 = (diff_1 - width).abs();
        diff_1.min(diff_2)
    }
    // We need the world to know its size because the world is circular
    pub fn sqr_distance(&self, other: &Person, world_width: f32, world_height: f32) -> f32 {
        let diff_x = Person::min_diff(self.position.x, other.position.x, world_width);
        let diff_y = Person::min_diff(self.position.y, other.position.y, world_height);
        diff_x * diff_x + diff_y * diff_y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dead_people_dont_move() {
        let rng = rand::thread_rng();
        let position = Location { x: 10.0, y: 10.0 };
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
            person.move_random(10.0, 100.0, 100.0);
            assert_eq!(person.position.x, 10.0);
            assert_eq!(person.position.y, 10.0);
        }
    }
}