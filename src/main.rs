#[test]
fn should_fail() {
    unimplemented!();
}

mod world;
use world::world::World;
use world::person::PersonState;
use world::virus::Virus;
use pixel_canvas::{Canvas, Color, input::MouseState, RC, image};
use std::time::{SystemTime};

fn color_pixel(image: &mut image::Image, x: isize, y: isize, state: &world::person::PersonState) {
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

fn main() {
    // Configure the window that you want to draw in. You can add an event
    // handler to build interactive art. Input handlers for common use are
    // provided.

    let mut world = World::new(1000, 1280, 720);
    world.config(15);
    let virus = Virus::corona();
    world.set_virus(&virus);

    let canvas = Canvas::new(world.get_width(), world.get_height())
        .title("World")
        .state(MouseState::new())
        .input(MouseState::handle_input);

    let mut now = SystemTime::now();
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

        for person in world.people.iter_mut() {
            color_pixel(image, person.position.x, person.position.y, &person.get_state())
        };
        world.update();
    });
}