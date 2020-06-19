use super::person::{Person, PersonState};
use super::virus::Virus;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, JsValue};

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub enum PopulationDistribution {
    Random,
    Grid,
}

#[wasm_bindgen]
pub struct Stats {
    pub susceptable: f32,
    pub infected: f32,
    pub recovered: f32,
}

#[wasm_bindgen]
pub struct World {
    people: Vec<Person>,
    width: usize,
    height: usize,
    move_speed: usize,
    virus: Virus,
}

#[wasm_bindgen]
impl World {
    /// Constructs a new World, the basic element in which we can put people and virussen.
    pub fn new(
        population: usize,
        width: usize,
        height: usize,
        virus: Virus,
        distribution: PopulationDistribution,
    ) -> World {
        let mut people: Vec<Person> = Vec::new();
        for index in 0..population {
            match distribution {
                PopulationDistribution::Random => {
                    people.push(Person::new_random(width, height, index as usize))
                }
                PopulationDistribution::Grid => {
                    let population = population as f32;
                    let grid_width = (population).sqrt().ceil();
                    let grid_height = (population / grid_width).floor();
                    let x = index % (grid_width as usize) * ((width as f32 / grid_width) as usize);
                    let y =
                        index / (grid_width as usize) * ((height as f32 / grid_height) as usize);
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
            virus,
        }
    }
    pub fn config(&mut self, move_speed: usize) {
        self.move_speed = move_speed;
    }
    /// # Returns the width of the world
    ///
    /// ```
    /// # use sir::sir::world::{PopulationDistribution, World};
    /// # use sir::sir::virus::Virus;
    /// let virus = Virus::corona();
    /// let world = World::new(1, 128, 256, virus, PopulationDistribution::Random);
    /// assert_eq!(world.get_width(), 128);
    /// ```
    pub fn get_width(&self) -> usize {
        self.width
    }
    /// # Returns the height of the world
    ///
    /// ```
    /// # use sir::sir::world::{PopulationDistribution, World};
    /// # use sir::sir::virus::Virus;
    /// let virus = Virus::corona();
    /// let world = World::new(1, 128, 256, virus, PopulationDistribution::Grid);
    /// assert_eq!(world.get_height(), 256);
    /// ```
    pub fn get_height(&self) -> usize {
        self.height
    }
    pub fn update(&mut self) {
        for person in self.people.iter_mut() {
            person.move_random(self.move_speed, self.width, self.height);
            person.update_age(self.virus.recovery_time, self.virus.mortality_rate);
        }
        let max_dist = self.virus.distance * self.virus.distance;
        self.infect_closeby(max_dist);
    }
    pub fn get_stats(&self) -> Stats {
        let mut count: (usize, usize, usize) = (0, 0, 0);
        for person in self.people.iter() {
            match person.get_state() {
                PersonState::Susceptible => count.0 += 1,
                PersonState::Infectious => count.1 += 1,
                PersonState::Recovered(_dead) => count.2 += 1,
            }
        }
        let total = (count.0 + count.1 + count.2) as f32;
        Stats {
            susceptable: count.0 as f32 / total,
            infected: count.1 as f32 / total,
            recovered: count.2 as f32 / total,
        }
    }
    pub fn render(&self, canvas_id: &str) {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id(canvas_id).unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();
        let red = "#ff0000";
        let green = "#00ff00";
        let blue = "#0000ff";
        let white = "#ffffff";
        let black = "#000000";
        context.set_fill_style(&JsValue::from_str(black));
        context.fill_rect(0.0, 0.0, self.width as f64, self.height as f64);
        for person in self.people.iter() {
            match person.get_state() {
                PersonState::Susceptible => context.set_fill_style(&JsValue::from_str(green)),
                PersonState::Infectious => context.set_fill_style(&JsValue::from_str(red)),
                PersonState::Recovered(false) => context.set_fill_style(&JsValue::from_str(blue)),
                PersonState::Recovered(true) => context.set_fill_style(&JsValue::from_str(white)),
            }
            context.fill_rect(
                (person.position.x - 1) as f64,
                (person.position.y - 1) as f64,
                3.0,
                3.0,
            );
        }
    }
}

impl World {
    pub fn people_mut(&mut self) -> &mut Vec<Person> {
        &mut self.people
    }
    fn infect_closeby(&mut self, max_dist: usize) {
        let max_dist: isize = max_dist as isize;
        // warning !! max number
        let mut to_infect = [false; 1_000_000];
        for i in 0..self.people.len() {
            for j in i + 1..self.people.len() {
                if self.people[i].get_state() == PersonState::Susceptible
                    && self.people[j].get_state() == PersonState::Infectious
                {
                    let dist =
                        self.people[i].sqr_distance(&self.people[j], self.width, self.height);
                    if dist < max_dist {
                        to_infect[self.people[i].get_id()] = true;
                    }
                } else if self.people[j].get_state() == PersonState::Susceptible
                    && self.people[i].get_state() == PersonState::Infectious
                {
                    let dist =
                        self.people[i].sqr_distance(&self.people[j], self.width, self.height);
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn update_move_speed() {
        let virus = Virus::corona();
        let mut world = World::new(1, 100, 100, virus, PopulationDistribution::Random);
        world.config(15);
        let mut person = world.people[0].clone();
        let mut max_move = 0;
        for _ in 1..10000 {
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
}
