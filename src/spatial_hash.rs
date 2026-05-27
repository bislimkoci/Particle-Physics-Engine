use std::collections::{HashMap};

use ggez::glam::Vec2;

use crate::{HEIGHT, WIDTH, particle::Particle, particle_container::ParticleContainer};

pub struct SpatialHash {
    pub gravity : f32,
    pub cell_size : f32,
    pub objects: Vec<Particle>,
    pub grid : HashMap<(i32, i32), Vec<usize>>,
}

impl SpatialHash {
    pub fn new() -> Self {
        SpatialHash { 
            gravity : 2.0,
            cell_size : 8.0, 
            objects : Vec::new(), 
            grid : HashMap::new(), }
    }


    fn get_grid_coords(&self, pos : Vec2) -> (i32, i32) {
        
        (
            (pos.x / self.cell_size).floor() as i32,
            (pos.y / self.cell_size).floor() as i32,
        )
    }

    fn rebuild_grid(&mut self) {
        self.grid.clear();
        for (idx, particle) in self.objects.iter().enumerate() {
            let coord = self.get_grid_coords(particle.position);
            self.grid.entry(coord).or_insert_with(Vec::new).push(idx);
        }
    }
}

impl ParticleContainer for SpatialHash {
    fn len(&self) -> usize {
        self.objects.len()
    }

    fn add(&mut self, particle: Particle) {
        self.objects.push(particle);
    }

    fn particles(&self) -> &Vec<Particle> {
        &self.objects
    }

    fn move_to_point(&mut self) {
        let center = Vec2::new(WIDTH/2.0, HEIGHT/2.0);
        for particle in &mut self.objects {
            particle.update_vel_to_point(&center);
        }
    }

    fn update_all(&mut self, dt : f32) {
        for particle in &mut self.objects {
            particle.update_vel(dt, &self.gravity);
            particle.step(dt);
            particle.is_out_of_bounds();
        }

        self.rebuild_grid();

        for _ in 0..3 {
            for i in 0..self.objects.len() {
                let p1_pos = self.objects[i].position;
                let center_coord = self.get_grid_coords(p1_pos);
            
                for dx in -1..=1 {
                    for dy in -1..=1 {
                        let neighbor_coord = (center_coord.0 + dx, center_coord.1 + dy);
                        
                        if let Some(neighbor_indices) = self.grid.get(&neighbor_coord) {
                            for &j in neighbor_indices {
                                if i >= j {
                                    continue;
                                }
                            
                                if i < j {
                                    let (left, right) = self.objects.split_at_mut(j);
                                    let p1 = &mut left[i];
                                    let p2 = &mut right[0];
                                    p1.collision(p2);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}