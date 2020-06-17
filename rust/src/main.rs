use sir::sir::world::{World, PopulationDistribution};
use sir::sir::virus::Virus;
// use ::rendering::canvas_render::render_world;
extern crate argparse;
use argparse::{ArgumentParser, StoreOption, Store};

fn main() {
    let mut request_width: Option<usize> = None;
    let mut request_height: Option<usize> = None;
    let mut request_population: Option<usize> = None;
    let mut population_distribution = "random".to_string();
    let graph_size = 200;
    let mut width = 1920;
    let mut population = 1000;
    let mut height = 1080 - graph_size;
    let mut distribution = PopulationDistribution::Random;
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("SIP Virus simulator");
        ap.refer(&mut request_width)
          .add_option(&["-w", "--width"], StoreOption, "Width of the world in pixels (default is 1920)");
        ap.refer(&mut request_height)
          .add_option(&["-h", "--height"], StoreOption, "Height of the world in pixels (default is 880)");
        ap.refer(&mut request_population)
           .add_option(&["-p", "--population"], StoreOption, "Number of people in the world (default is 1000)");
        ap.refer(&mut population_distribution)
           .add_option(&["-d", "--distribution"], Store, "Distribution of people in the world (random or grid)");
        ap.parse_args_or_exit();
    }
    if let Some(value) = request_width {
        width = std::cmp::min(value, 1920);
    }
    if let Some(value) = request_height {
        height = std::cmp::min(value, 1080 - graph_size);
    }
    if let Some(value) = request_population {
        population = std::cmp::min(value, 1_000_000);
        if population < 1 {
            population = 1;
        }
    }
    if population_distribution == "grid" {
        distribution = PopulationDistribution::Grid;
    }
    let virus = Virus::corona();
    let mut world = World::new(population, width, height, virus, distribution);
    world.config(15);
    // render_world(world, width, height, graph_size);
}