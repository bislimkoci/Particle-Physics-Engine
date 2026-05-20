use ggez::glam::Vec2;

pub struct Particle {
    pub position : Vec2,
    pub velocity : Vec2,
    pub radius : f32,
    gravity : f32
}

impl Particle {
    pub fn new() -> Particle {
        Particle { position: Vec2 { x: 640.0, y: 360.0 }, velocity: Vec2 { x: 360.0, y: -840.0 }, radius: 20.0, gravity: 982.0}
    }

    pub fn step(&mut self, dt : f32) {
        self.position.x += self.velocity.x * dt;
        self.position.y += self.velocity.y * dt;
    }

    pub fn is_out_of_bounds(&mut self, width : f32, height : f32) {
        let x = self.position.x;
        let y = self.position.y;
        let r = self.radius;

        let damp : f32 = 0.7;

        if x - r <= 0.0 && self.velocity.x < 0.0 {
            self.position.x = r;
            self.velocity.x *= -1.0 * damp; 
        }

        if x + r >= width && self.velocity.x > 0.0 {
            self.position.x = width - r;
            self.velocity.x *= -1.0 * damp;
        }

        if y - r <= 0.0 && self.velocity.y < 0.0 {
            self.position.y = r;
            self.velocity.y *= -1.0 * damp; 
        }

        if y + r >= height && self.velocity.y > 0.0 {
            self.position.y = height - r;
            self.velocity.y *= -1.0 * damp;  
        }
    }

    pub fn update_vel(&mut self, dt : f32) {
        self.velocity.y += self.gravity * dt;
    }

    pub fn collision(&mut self , other : &mut Particle) {
        
        //dist to see if two particles collide
        let dist = self.position.distance(other.position);
        let total_radius = self.radius + other.radius;

        if dist <= total_radius {
            
            let overlap = total_radius - dist;

            //Needed so particles don't get stuck on each other right on the edge
            let epsilon = 0.05;
            let adjusted_overlap = overlap + epsilon;

            //Used to get the direction the particles collide from
            let normal = if dist > 0.0 {
                (other.position - self.position) / dist
            } else {
                //Just push somewhere if they are exactly on the same point
                Vec2 { x: 1.0, y: 0.0 }
            };
            
            // this gives the correct directional shift with the given magnitude
            let push_vector = normal * (adjusted_overlap / 2.0);
            
            //push in each direction
            self.position -= push_vector;
            other.position += push_vector;
            
            
            let damp: f32 = 0.9;

            // Take only account on how much the particles hit each other
            // since particles don't often hit head on
            let relative_velocity = other.velocity - self.velocity;
            let velocity_along_normal = relative_velocity.dot(normal);

            //check if they are heading towards each other
            if velocity_along_normal < 0.0 {
                // value 2 comes from the formula 1/m1 + 1/m2, where m1=1 and m2=1
                let impulse = -(1.0 + damp) * velocity_along_normal /2.0;

                //Need to multiply with normal to get the direction of the velocity
                self.velocity -= normal * impulse;
                other.velocity += normal * impulse;
            }

        }

    }


}