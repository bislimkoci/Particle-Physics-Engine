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

        self.particles.update_all(dt_seconds);
        
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
    let min_vel_col = [20.0, 50.0, 200.0];  // Blue
    let mid_vel_col = [200.0, 50.0, 200.0];  // Purple
    let max_vel_col = [200.0, 50.0, 20.0];   // Red

    let max_vel = 1400.0;
    let speed = particle.velocity.length().min(max_vel);

    let t = speed / max_vel;

    let final_color = if t < 0.5 {
        //t from [0.0, 0.5] to [0.0, 1.0]
        let local_t = t * 2.0; 
        lerp_color(min_vel_col, mid_vel_col, local_t)
    } else {
        // t from [0.5, 1.0] to [0.0, 1.0]
        let local_t = (t - 0.5) * 2.0; 
        lerp_color(mid_vel_col, max_vel_col, local_t)
    };

    fn lerp_color(color1: [f32; 3], color2: [f32; 3], t: f32) -> [f32; 3] {
        [
            color1[0] + (color2[0] - color1[0]) * t,
            color1[1] + (color2[1] - color1[1]) * t,
            color1[2] + (color2[2] - color1[2]) * t,
        ]
    }

        
    Color::from_rgb(final_color[0] as u8, final_color[1] as u8, final_color[2] as u8)

}
