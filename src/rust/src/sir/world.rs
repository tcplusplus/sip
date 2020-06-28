use super::person::{Person, PersonState};
use super::virus::Virus;
use super::population::Population;
use std::iter::Flatten;
use std::slice::Iter;
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
    width: f32,
    height: f32,
    move_speed: f32,
    population: Population
}

#[wasm_bindgen]
impl World {
    /// Constructs a new World, the basic element in which we can put people and virussen.
    pub fn new(
        population_size: usize,
        width: f32,
        height: f32,
        virus: Virus,
        distribution: PopulationDistribution,
    ) -> World {
        let max_dist = virus.distance;
        let mut num_grid_width: usize = ((width as f32) / max_dist).floor() as usize;
        let mut num_grid_height: usize = ((height as f32) / max_dist).floor() as usize;
        if num_grid_width > 1000 {
            num_grid_width = 1000;
        }
        if num_grid_height > 1000 {
            num_grid_height = 1000;
        }
        let mut population = Population::new(width as f32, height as f32, num_grid_width, num_grid_height);
        for index in 0..population_size {
            let mut person = match distribution {
                PopulationDistribution::Random => {
                    Person::new_random(width, height, index as usize)
                }
                PopulationDistribution::Grid => {
                    let population = population_size as f32;
                    let grid_width = (population).sqrt().ceil();
                    let grid_height = (population / grid_width).floor();
                    let x = index % (grid_width as usize) * ((width as f32 / grid_width) as usize);
                    let y =
                        index / (grid_width as usize) * ((height as f32 / grid_height) as usize);
                    Person::new(x as f32, y as f32, index as usize)
                }
            };
            if index == 0 {
                person.infect(virus.clone());
            }
            population.add(person);
        }

        World {
            population,
            width,
            height,
            move_speed: 5.0
        }
    }
    pub fn config(&mut self, move_speed: f32) {
        self.move_speed = move_speed;
    }
    /// # Returns the width of the world
    ///
    /// ```
    /// # use sir::sir::world::{PopulationDistribution, World};
    /// # use sir::sir::virus::Virus;
    /// let virus = Virus::corona();
    /// let world = World::new(1, 128.0, 256.0, virus, PopulationDistribution::Random);
    /// assert_eq!(world.get_width(), 128.0);
    /// ```
    pub fn get_width(&self) -> f32 {
        self.width
    }
    /// # Returns the height of the world
    ///
    /// ```
    /// # use sir::sir::world::{PopulationDistribution, World};
    /// # use sir::sir::virus::Virus;
    /// let virus = Virus::corona();
    /// let world = World::new(1, 128.0, 256.0, virus, PopulationDistribution::Grid);
    /// assert_eq!(world.get_height(), 256.0);
    /// ```
    pub fn get_height(&self) -> f32 {
        self.height
    }
    pub fn update(&mut self) {
        self.population.update_positions(self.move_speed);
        self.population.infect_closeby();
    }
    pub fn get_stats(&self) -> Stats {
        let mut count: (usize, usize, usize) = (0, 0, 0);
        for person in self.population.iter() {
            match person.get_state() {
                PersonState::Susceptible => count.0 += 1,
                PersonState::Infectious(_virus) => count.1 += 1,
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
        for person in self.population.iter() {
            match person.get_state() {
                PersonState::Susceptible => context.set_fill_style(&JsValue::from_str(green)),
                PersonState::Infectious(_virus) => context.set_fill_style(&JsValue::from_str(red)),
                PersonState::Recovered(false) => context.set_fill_style(&JsValue::from_str(blue)),
                PersonState::Recovered(true) => context.set_fill_style(&JsValue::from_str(white)),
            }
            context.fill_rect(
                (person.position.x - 1.0) as f64,
                (person.position.y - 1.0) as f64,
                2.0,
                2.0,
            );
        }
    }
}

impl World {
    pub fn people(&self) -> std::iter::Flatten<Flatten<Iter<'_, Vec<Vec<Person>>>>> {
        self.population.iter()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn update_move_speed() {
        let virus = Virus::corona();
        let mut world = World::new(1, 100.0, 100.0, virus, PopulationDistribution::Random);
        world.config(15.0);
        println!("Hier");
        let mut person = world.population.iter().next().unwrap().clone();
        let mut max_move = 0.0;
        for _ in 1..10000 {
            println!("before update");
            world.update();
            println!("after update");
            let dist = person.sqr_distance(&world.population.iter().next().unwrap(), 100.0, 100.0);
            println!("dist {}", dist);
            if dist > max_move {
                max_move = dist;
            }
            person = world.population.iter().next().unwrap().clone();
        }
        // There is a random factor in here
        println!("Maximum max distance is {}", max_move);
        assert!(max_move < 450.0 && max_move > 400.0);
    }
}
