use nannou::prelude::*;

use crate::cursor::CursorMode;

#[derive(PartialEq)]
pub struct Boid {
    pub width: f32,
    pub height: f32,
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub max_force: f32,
    pub max_speed: f32,
    pub min_speed: f32,
    pub visual_range: f32,
    pub protected_range: f32,
}

impl Boid {
    pub fn new(x: f32, y: f32) -> Boid {
        let width = 10.0;
        let height = 15.0;
        let position = vec2(x, y);
        let velocity = vec2(random_range(-1.0, 1.0), random_range(-1.0, 1.0));
        let acceleration = vec2(0.0, 0.0);
        let max_force = 0.2;
        let max_speed = 4.0;
        let min_speed = 2.0;
        let visual_range = 100.0;
        let protected_range = 30.0;
        Boid {
            width,
            height,
            position,
            velocity,
            acceleration,
            max_force,
            max_speed,
            min_speed,
            visual_range,
            protected_range,
        }
    }

    pub fn align(&self, nearby_boids: &[&Boid]) -> Vec2 {
        let mut average_alignment = Vec2::ZERO;
        if nearby_boids.is_empty() {
            return average_alignment;
        }

        // Get Average Velocity
        for other in nearby_boids {
            average_alignment += other.velocity;
        }
        average_alignment /= nearby_boids.len() as f32;

        average_alignment.clamp_length_max(1.0)
    }

    pub fn separate(&self, nearby_boids: &[&Boid]) -> Vec2 {
        let mut separation_force = Vec2::ZERO;
        if nearby_boids.is_empty() {
            return separation_force;
        }

        // Get Average Velocity
        for boid in nearby_boids {
            let distance_vec = self.position - boid.position;
            let length = distance_vec.length();
            let weight = (self.protected_range - length) / self.protected_range;

            separation_force += distance_vec.clamp_length_max(1.0) * weight;
        }

        separation_force
    }

    pub fn cohere(&self, nearby_boids: &[&Boid]) -> Vec2 {
        let mut cohesion_force = Vec2::ZERO;
        let mut average_position = Vec2::ZERO;
        if nearby_boids.is_empty() {
            return cohesion_force;
        }

        // Get Average Velocity
        for other in nearby_boids {
            average_position += other.position;
        }

        average_position /= nearby_boids.len() as f32;
        cohesion_force = (average_position - self.position).normalize();
        cohesion_force
    }

    pub fn wrap(&mut self, win: &Rect) {
        let left = win.left();
        let right = win.right();
        let top = win.top();
        let bottom = win.bottom();

        self.position.x = match self.position.x {
            x if x > right => left,
            x if x < left => right,
            x => x,
        };
        self.position.y = match self.position.y {
            y if y > top => bottom,
            y if y < bottom => top,
            y => y,
        };
    }

    pub fn avoid_bounds(&mut self, win: &Rect) -> Vec2 {
        let margin = 50.0;
        let left = win.left() + self.visual_range + margin;
        let right = win.right() - self.visual_range - margin;
        let top = win.top() - self.visual_range - margin;
        let bottom = win.bottom() + self.visual_range + margin;

        let desired = match self.position.to_array() {
            [x, _] if x < left => Some(vec2(self.max_speed, self.velocity.y)),
            [x, _] if x > right => Some(vec2(-self.max_speed, self.velocity.y)),
            [_, y] if y < bottom => Some(vec2(self.velocity.x, self.max_speed)),
            [_, y] if y > top => Some(vec2(self.velocity.x, -self.max_speed)),
            _ => None,
        };
        if let Some(desired) = desired {
            let desired = desired.normalize() * self.max_speed;
            let steer = (desired - self.velocity).clamp_length_max(self.max_force * 1.5);
            return steer;
        }
        Vec2::ZERO
    }

    pub fn get_neighbours<'a>(&self, flock: &'a Vec<Boid>) -> (Vec<&'a Boid>, Vec<&'a Boid>) {
        let mut nearby_boids: Vec<&Boid> = Vec::new();
        let mut close_boids: Vec<&Boid> = Vec::new();
        for other in flock {
            let d = self.position.distance(other.position).abs();
            if other != self && d <= self.visual_range && d > self.protected_range {
                nearby_boids.push(other);
            }
            if other != self && d < self.protected_range {
                close_boids.push(other);
            }
        }
        (nearby_boids, close_boids)
    }

    pub fn cursor_interaction(&mut self, app: &App, cursor_mode: &CursorMode) -> Vec2 {
        let cursor_pos = app.mouse.position();
        let direction = match cursor_mode {
            CursorMode::Attract => 1.0,
            CursorMode::Avoid => -1.0,
            CursorMode::Ignore => 0.0,
        };
        if self.position.distance(cursor_pos) > self.visual_range * 2.0 {
            return Vec2::ZERO;
        }

        ((cursor_pos - self.position) * direction).clamp_length_max(1.5) // set the desired speed
    }

    pub fn update(&mut self) {
        self.velocity += self.acceleration;
        // Limit speed between bounds
        self.velocity = self.velocity.clamp_length(self.min_speed, self.max_speed);

        //TODO: Implement deltatime when reliable refresh rate can be used.
        // self.velocity *= delta_time;
        self.position += self.velocity;
        // Reset acceleration to 0 each cycle.
        self.acceleration *= 0.0;
    }
    pub fn show(&self, draw: &Draw) {
        draw.tri()
            .w_h(self.height, self.width)
            .xy(self.position)
            .rotate(self.velocity.angle())
            .color(BLACK);
    }
}
