use ggez::{ graphics::{Color, Mesh}, winit::{keyboard::{KeyCode, PhysicalKey}}, *};

use crate::{particle::Particle, particle_container::ParticleContainer};

pub struct State {
    pub dt : std::time::Duration,
    pub particles : Box<dyn ParticleContainer>,
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

        for particle in self.particles.particles() {
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
                let len = self.particles.len();
                println!("Nbr Particles: {len}");
            },
            PhysicalKey::Code(KeyCode::KeyC) => {
                self.particles.move_to_point();
            },
            _ => (),

        }

        Ok(())
    }

}


fn mesh_circle_particle(particle : &Particle, _ctx: &mut Context) -> Mesh {
    let color = color_from_velocity(particle);

    let circle = graphics::Mesh::new_circle(
            _ctx,
            graphics::DrawMode::fill(),
            particle.position,
            particle.radius,
            0.1,
            color,
        ).unwrap();
    circle
}

fn color_from_velocity(particle : &Particle) -> Color{
    let speed = particle.velocity.length();

    let color_ramp: [(f32, (u8, u8, u8)); 4] = [
        (100.0, (2, 62, 138)),
        (300.0, (129, 106, 212)),
        (600.0, (226, 72, 250)),
        (1000.0, (165, 15, 21)),
    ];

    for &(threshold, color) in &color_ramp {
        if speed < threshold {
            return Color::from_rgb(color.0, color.1, color.2);
        }
    }

    let col = color_ramp.get(3).unwrap().1;
    Color::from_rgb(col.0, col.1, col.2)

}