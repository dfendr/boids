use std::ops::Add;

use nannou::prelude::*;

const N_BOIDS: u32 = 100;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
    flock: Vec<Boid>,
}

struct Flock;
impl Flock {
    fn new_flock(bounds: Rect, n_boids: u32) -> Vec<Boid> {
        let (left, right, bottom, top) = bounds.l_r_b_t();
        let mut flock: Vec<Boid> = Vec::new();
        for _ in 0..n_boids {
            let x_pos = random_range(left, right);
            let y_pos = random_range(bottom, top);
            let boid = Boid::new(x_pos, y_pos);
            flock.push(boid);
        }
        flock
    }
}

#[derive(PartialEq)]
struct Boid {
    width: f32,
    height: f32,
    position: Vec2,
    velocity: Vec2,
    acceleration: Vec2,
    max_force: f32,
    max_speed: f32,
    min_speed: f32,
    visual_range: f32,
    protected_range: f32,
}

impl Boid {
    fn new(x: f32, y: f32) -> Boid {
        let width = 10.0;
        let height = 15.0;
        let position = vec2(x, y);
        let velocity = vec2(random_range(-1.0, 1.0), random_range(-1.0, 1.0));
        let acceleration = vec2(0.0, 0.0);
        let max_force = 0.2;
        let max_speed = 5.0;
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

    fn align(&self, nearby_boids: &[&Boid]) -> Vec2 {
        let mut steering = Vec2::ZERO;
        if nearby_boids.is_empty() {
            return steering;
        }

        // Get Average Velocity
        for other in nearby_boids {
            steering += other.velocity;
        }
        steering /= nearby_boids.len() as f32;

        // Scale accordingly
        steering = (steering - self.velocity) * self.max_force;

        steering
    }

    fn separate(&self, nearby_boids: &[&Boid]) -> Vec2 {
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

    fn cohere(&self, nearby_boids: &[&Boid]) -> Vec2 {
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

    fn apply_force(&mut self, force: Vec2) {
        // We could add mass here if we want A = F / M
        self.acceleration += force;
    }

    fn wrap(&mut self, win: &Rect) {
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

    fn avoid_bounds(&mut self, win: &Rect) {
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
            self.apply_force(steer);
        }
    }

    fn nearby_boids<'a>(&self, flock: &'a Vec<Boid>) -> (Vec<&'a Boid>, Vec<&'a Boid>) {
        let mut nearby_boids: Vec<&Boid> = Vec::new();
        let mut close_boids: Vec<&Boid> = Vec::new();
        for other in flock {
            let d = self.position.distance(other.position).abs();
            if other != self && d <= self.visual_range && d > self.protected_range {
                nearby_boids.push(other);
            }
            if d < self.protected_range {
                close_boids.push(other);
            }
        }
        (nearby_boids, close_boids)
    }

    fn update(&mut self) {
        self.velocity += self.acceleration;
        // Limit speed
        self.velocity = self.velocity.clamp_length(self.min_speed, self.max_speed);
        self.position += self.velocity;
        // Reset acceleration to 0 each cycle.
        self.acceleration *= 0.0;
    }
}

fn limit_2d(vec2: Vec2, limit: f32) -> Vec2 {
    let length = magnitude_2d(vec2);
    if length > limit {
        return vec2.normalize() * limit;
    }
    vec2
}
fn magnitude_2d(vec2: Vec2) -> f32 {
    (vec2.x * vec2.x + vec2.y * vec2.y).sqrt()
}

fn set_magnitude_2d(vec2: Vec2, magnitude: f32) -> Vec2 {
    vec2.normalize() * magnitude
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .title(app.exe_name().unwrap_or_default())
        .view(view)
        .build()
        .expect("ah geez something happened...I'm sorry.");
    let flock = Flock::new_flock(app.window_rect(), N_BOIDS);
    Model { _window, flock }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    for i in 0..model.flock.len() {
        let (nearby_boids, close_boids) = model.flock[i].nearby_boids(&model.flock);
        let v1 = model.flock[i].align(&nearby_boids);
        let v2 = model.flock[i].separate(&close_boids);
        let v3 = model.flock[i].cohere(&nearby_boids);
        model.flock[i].acceleration += v1;
        model.flock[i].acceleration += v2;
        model.flock[i].acceleration += v3;
        model.flock[i].avoid_bounds(&app.window_rect());
        model.flock[i].wrap(&app.window_rect());
        model.flock[i].update();
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(SNOW);

    for boid in &model.flock {
        // for other in boid.nearby_boids(&model.flock) {
        //     draw.line()
        //         .start(boid.position)
        //         .end(other.position)
        //         .color(RED);
        // }
        draw.tri()
            .w_h(boid.height, boid.width)
            .xy(boid.position)
            .rotate(boid.velocity.angle())
            .color(BLACK);
    }
    draw.to_frame(app, &frame)
        .expect("WHOA NELLY THIS AINT GOOD");
}
