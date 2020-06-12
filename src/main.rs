mod sir;
use sir::world::{World, PopulationDistribution};
use sir::person::PersonState;
use sir::virus::Virus;
use pixel_canvas::{Canvas, Color, input::MouseState, RC, image};
use std::time::{SystemTime};
extern crate argparse;
use argparse::{ArgumentParser, StoreOption, Store};

// TODO move to new class
fn color_pixel(image: &mut image::Image, x: isize, y: isize, state: &PersonState) {
    let width = image.width() as isize;
    let height = image.height() as isize;
    let size = 1;
    for i in y-size..y+size+1 {
        if i >= 0 && i < height {
            for j in x-size..x+size+1 {
                if j >= 0 && j < width {
                    let mut pix: &mut Color = &mut image[RC(i as usize, j as usize)];
                    match state {
                        PersonState::Susceptible => pix.g = 255,
                        PersonState::Infectious => pix.r = 255,
                        PersonState::Recovered(false) => pix.b = 255,
                        PersonState::Recovered(true) => {
                            pix.r = 255;
                            pix.g = 255;
                            pix.b = 255;
                        }
                    }
                }
            }
        }
    }
}

// TODO move to new class
fn plot_graph(image: &mut image::Image, stats: &[(f32, f32, f32)], line_start: usize, line_end: usize) {
    let width = image.width() as usize;
    for (y, row) in image.chunks_mut(width).enumerate() {
        for (x, pixel) in row.iter_mut().enumerate() {
            if y > line_start {
                let mut r = 255;
                let mut g = 255;
                let mut b = 255;
                if x < stats.len() {
                    let percent = 1.0 - ((line_end as f32) - (y as f32)) / ((line_end as f32) - (line_start as f32));
                    if stats[x].1 > percent {
                        b = 0;
                        g = 0;
                    } else if stats[x].0 + stats[x].1 > percent {
                        r = 0;
                        b = 0;
                    } else {
                        r = 0;
                        g = 0;
                    }
                }
                *pixel = Color { r, g, b };
            }

        }
    }
}

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

    let canvas = Canvas::new(world.get_width(), world.get_height() + graph_size)
        .title("World")
        .state(MouseState::new())
        .input(MouseState::handle_input);

    let mut now = SystemTime::now();
    let mut stats: Vec<(f32, f32, f32)> = Vec::new();
    // The canvas will render for you at up to 60fps.
    canvas.render(move |_mouse, image| {
        match now.elapsed() {
            Ok(elapsed) => {
                println!("{} ms", elapsed.as_millis());
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
        now = SystemTime::now();
        image.fill(Color {r: 0, g: 0, b: 0});

        let mut count: (usize, usize, usize) = (0, 0, 0);
        for person in world.people_mut().iter_mut() {
            color_pixel(image, person.position.x, person.position.y, &person.get_state());
            match person.get_state() {
                PersonState::Susceptible => count.0 += 1,
                PersonState::Infectious => count.1 += 1,
                PersonState::Recovered(_dead) => count.2 += 1
            }
        };
        let total = (count.0 + count.1 + count.2) as f32;
        stats.push(((count.0 as f32) / total, (count.1 as f32) / total, (count.2 as f32) / total));
        //println!(">> {:?} ", stats);
        plot_graph(image, &stats, world.get_height(), world.get_height() + graph_size);
        world.update();
    });
}