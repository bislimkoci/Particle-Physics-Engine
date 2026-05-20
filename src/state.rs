use ggez::{ graphics::{Color, Mesh}, winit::{keyboard::{KeyCode, PhysicalKey}}, *};

use crate::particle::{Particle};
use crate::particle_array::{ParticleArray};

pub struct State {
    pub dt : std::time::Duration,
    pub particles : ParticleArray,
}


impl ggez::event::EventHandler for State {
    fn update(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        self.dt = _ctx.time.delta();
        
        let dt_seconds = self.dt.as_secs_f32();

        self.particles.update_all(dt_seconds, _ctx.fields.conf.window_mode.width, _ctx.fields.conf.window_mode.height);
        
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        
        let mut canvas = graphics::Canvas::from_frame(_ctx, Color::BLACK);

        for particle in &self.particles.particles {
            let mesh = mesh_circle_particle(particle, _ctx);
            canvas.draw(&mesh, graphics::DrawParam::default());
        }

        canvas.finish(_ctx)?;

        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, input: input::keyboard::KeyInput, _repeated: bool) -> Result<(), GameError> {
        match input.event.physical_key {

            PhysicalKey::Code(KeyCode::KeyA) => {
                self.particles.add(Particle::new());
            }
            _ => (),

        }

        Ok(())
    }

}


fn mesh_circle_particle(particle : &Particle, _ctx: &mut Context) -> Mesh {
    let circle = graphics::Mesh::new_circle(
            _ctx,
            graphics::DrawMode::fill(),
            particle.position,
            particle.radius,
            0.1,
            Color::WHITE,
        ).unwrap();
    circle
}
