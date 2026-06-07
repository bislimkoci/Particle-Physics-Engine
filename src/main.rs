mod particle;
mod particle_container;
mod spatial_hash_2;
mod state;

use ggez::{
    conf::{WindowMode, WindowSetup},
    event, ContextBuilder,
};
use spatial_hash_2::SpatialHash;
use state::State;

const WIDTH: f32 = 1440.0;
const HEIGHT: f32 = 720.0;
const GRAVITY: f32 = 1000.0;
const PARTICLE_RADIUS: f32 = 8.0;

fn main() {
    let state = State {
        dt: std::time::Duration::new(0, 0),
        particles: Box::new(SpatialHash::new()),
    };

    let (ctx, event_loop) = ContextBuilder::new("hello_ggez", "awesome_person")
        .window_mode(
            WindowMode::default()
                .dimensions(WIDTH, HEIGHT)
                .resizable(false)
                .max_dimensions(WIDTH, HEIGHT)
                .min_dimensions(WIDTH, HEIGHT),
        )
        .window_setup(WindowSetup::default().title("2D Particle simulation!!!"))
        .build()
        .unwrap();

    event::run(ctx, event_loop, state).unwrap();
}
