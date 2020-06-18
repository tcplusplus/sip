use pixel_canvas::{Canvas, Color, input::MouseState, RC, image};
use std::time::{SystemTime};
use crate::sir::world::World;
use crate::sir::person::PersonState;

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

pub fn render_world(mut world: World, width: usize, height: usize, graph_size: usize) {
    let canvas = Canvas::new(width, height + graph_size)
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
        for person in world.people_mut().iter_mut() {
            color_pixel(image, person.position.x, person.position.y, &person.get_state());
        };
        stats.push(world.get_stats());
        plot_graph(image, &stats, world.get_height(), world.get_height() + graph_size);
        world.update();
    });
}