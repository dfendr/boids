use nannou::prelude::*;

use crate::boids::{Boid, BoidType};

pub struct Flock;
impl Flock {
    pub fn new_flock(bounds: Rect, n_boids: u32, boid_type: BoidType) -> Vec<Boid> {
        let (left, right, bottom, top) = bounds.l_r_b_t();
        let mut flock: Vec<Boid> = Vec::new();
        for _ in 0..n_boids {
            let x_pos = random_range(left, right);
            let y_pos = random_range(bottom, top);
            let boid = Boid::new(x_pos, y_pos, boid_type);
            flock.push(boid);
        }
        flock
    }
}
