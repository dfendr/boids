use nannou::prelude::*;

use crate::{cursor::CursorMode, model::Model, theme::Theme};

#[derive(PartialEq, Clone, Copy)]
pub enum BoidType {
    Prey,
    Predator,
}

impl BoidType {
    fn default_color(self) -> Rgb8 {
        match self {
            BoidType::Prey => BLACK,
            BoidType::Predator => DARKRED,
        }
    }

    fn visual_range(self) -> f32 {
        match self {
            BoidType::Prey => 80.0,
            BoidType::Predator => 100.0,
        }
    }

    fn max_speed(self) -> f32 {
        match self {
            BoidType::Prey => 4.0,
            BoidType::Predator => 3.0,
        }
    }
    fn size(self) -> (f32, f32) {
        match self {
            BoidType::Prey => (10.0, 12.0),
            BoidType::Predator => (15.0, 17.0),
        }
    }
}

#[derive(PartialEq)]
pub struct Boid {
    pub b_type: BoidType,
    pub color: Rgb8,
    pub width: f32,
    pub height: f32,
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub neighbour_count: usize,
    pub max_force: f32,
    pub max_speed: f32,
    pub min_speed: f32,
    pub visual_range: f32,
    pub protected_range: f32,
}

impl Boid {
    pub fn new(x: f32, y: f32, boid_type: BoidType) -> Boid {
        let color = boid_type.default_color();
        let (width, height) = boid_type.size();
        let position = vec2(x, y);
        let velocity = vec2(random_range(-1.0, 1.0), random_range(-1.0, 1.0));
        let acceleration = vec2(0.0, 0.0);
        let neighbour_count = 0;
        let max_force = 0.2;
        let max_speed = boid_type.max_speed();
        let min_speed = 2.0;
        let visual_range = boid_type.visual_range();
        let protected_range = 30.0;
        Boid {
            b_type: boid_type,
            color,
            width,
            height,
            position,
            velocity,
            neighbour_count,
            acceleration,
            max_force,
            max_speed,
            min_speed,
            visual_range,
            protected_range,
        }
    }

    pub fn separate(&self, nearby_boids: &[&Boid]) -> Vec2 {
        nearby_boids.iter().fold(Vec2::ZERO, |sum, &boid| {
            if self.b_type == boid.b_type {
                let distance_vec = self.position - boid.position;
                let length = distance_vec.length();
                let weight = (self.protected_range - length) / self.protected_range;

                return sum + distance_vec.normalize_or_zero() * weight;
            }
            sum
        })
    }
    #[allow(clippy::cast_precision_loss)]
    pub fn cohere(&self, nearby_boids: &[&Boid]) -> Vec2 {
        let average_position = nearby_boids
            .iter()
            .fold(Vec2::ZERO, |sum, &boid| sum + boid.position);

        if average_position == Vec2::ZERO {
            average_position
        } else {
            // precision loss no big deal when nearby_boids is <=1000
            let len = nearby_boids.len() as f32;
            (average_position / len - self.position).normalize()
        }
    }

    pub fn wrap(&mut self, win: &Rect) {
        let pad = 10.0;
        let left = win.left() - pad;
        let right = win.right() + pad;
        let top = win.top() + pad;
        let bottom = win.bottom() - pad;

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

    pub fn close_predators<'a>(&self, predators: &'a Vec<Boid>) -> Vec<&'a Boid> {
        let mut close_predators: Vec<&Boid> = Vec::new();
        for other in predators {
            let d = self.position.distance(other.position).abs();
            if other != self && d < self.protected_range {
                close_predators.push(other);
            }
        }
        close_predators
    }

    pub fn avoid_predators(&self, predators: &[Boid]) -> Vec2 {
        let average_position = predators
            .iter()
            .filter(|predator| predator.position.distance(self.position) < self.visual_range)
            .fold(Vec2::ZERO, |sum, predator| sum + predator.position);

        if average_position == Vec2::ZERO {
            average_position
        } else {
            ((average_position - self.position) * -1.0).clamp_length_max(0.7)
        }
    }

    pub fn cursor_interaction(&self, app: &App, cursor_mode: &CursorMode) -> Vec2 {
        let cursor_pos = app.mouse.position();
        let (direction, range_modifier) = match cursor_mode {
            CursorMode::Attract => (1.0, 2.0),
            CursorMode::Avoid => (-1.0, 1.0),
            CursorMode::Ignore => (0.0, 0.0),
        };
        if self.position.distance(cursor_pos) > self.visual_range * range_modifier {
            return Vec2::ZERO;
        }

        ((cursor_pos - self.position) * direction).normalize_or_zero() // set the desired speed
    }

    pub fn update(&mut self) {
        self.velocity += self.acceleration;
        // Limit speed between bounds
        self.velocity = self.velocity.clamp_length(self.min_speed, self.max_speed);

        self.position += self.velocity;
        // Reset acceleration to 0 each cycle.
        self.acceleration *= 0.0;
    }
    pub fn show(&self, draw: &Draw, model: &Model) {
        // let flock_count = model.flock.len();
        // let ratio = self.neighbour_count as f32 / flock_count as f32;
        // let color = get_color(ratio, &model.theme, &self.color);
        let mut color = self.color;
        if model.theme == Theme::DeepSea {
            color = rgb8(200, 200, 200);
        }

        draw.tri()
            .w_h(self.height, self.width)
            .xy(self.position)
            .rotate(self.velocity.angle())
            .color(color);
    }
}

pub fn align(nearby_boids: &[&Boid]) -> Vec2 {
    nearby_boids
        .iter()
        .fold(Vec2::ZERO, |sum, &boid| sum + boid.velocity)
        .normalize_or_zero()
}
