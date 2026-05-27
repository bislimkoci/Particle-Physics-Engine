use crate::particle::Particle;

pub trait ParticleContainer {
    fn len(&self) -> usize;

    fn move_to_point(&mut self);

    fn update_all(&mut self, dt: f32);

    fn add(&mut self, particle : Particle);

    fn particles(&self) -> &Vec<Particle>;
}