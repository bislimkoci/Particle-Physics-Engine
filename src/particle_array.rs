use crate::particle::{Particle};

pub struct ParticleArray {
    pub particles : Vec<Particle>,
}


impl ParticleArray {
    pub fn new() -> ParticleArray {
        ParticleArray { particles: Vec::new() }
    }

    pub fn add(&mut self, particle : Particle) {
        self.particles.push(particle);
    }

    pub fn update_all(&mut self, dt: f32, width: f32, height: f32) {
        for particle in &mut self.particles {
            particle.update_vel(dt);
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

}

