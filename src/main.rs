mod state;
mod particle;
mod particle_array;
mod particle_container;

use ggez::{ContextBuilder, conf::{WindowMode, WindowSetup}, event};
use state::State;

fn main() {
    let state = State{
        dt: std::time::Duration::new(0, 0),
        particles: Box::new(crate::particle_array::ParticleArray::new()),
    };

        

    let (ctx, event_loop) = ContextBuilder::new("hello_ggez", "awesome_person")
        .window_mode(
            WindowMode::default()
                .dimensions(1440.0, 720.0)   
                .resizable(true)
                .max_dimensions(1280.0, 720.0)
        )
        .window_setup(
            WindowSetup::default().title("2D Particle simulation!!!")
        )
        .build()
        .unwrap();


    event::run(ctx, event_loop, state).unwrap();
}
