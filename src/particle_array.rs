use ggez::glam::Vec2;

use crate::particle::{Particle};
use crate::particle_container::{ParticleContainer};

pub struct ParticleArray {
    pub particles : Vec<Particle>,
    pub gravity : f32,
}

impl ParticleArray {
    pub fn new() -> ParticleArray {
        ParticleArray { particles: Vec::new(), gravity : 980.0}
    }
}

impl ParticleContainer for ParticleArray {

    fn add(&mut self, particle : Particle) {
        self.particles.push(particle);
    }

    fn update_all(&mut self, dt: f32, width: f32, height: f32) {
        for particle in &mut self.particles {
            particle.update_vel(dt, &self.gravity);
            particle.step(dt);
        }

        
        let iterations = 6;
        
        for _ in 0..iterations {
            let len = self.particles.len();
            for i in 0..len {
                let (left, right) = self.particles.split_at_mut(i + 1);
                let p1 = &mut left[i];
    
                for p2 in right {
                    p1.collision(p2);
                }
            }
            
            for particle in &mut self.particles {
                particle.is_out_of_bounds(width, height);
            }
        }
        

    }

    fn move_to_point(&mut self) {
        let point = Vec2{x: 640.0, y: 360.0 };
        for particle in &mut self.particles {
            particle.update_vel_to_point(&point);
        }
    }

    fn particles(&self) -> &Vec<Particle> {
        &self.particles
    }

    fn len(&self) -> usize {
        self.particles.len()
    }

}

