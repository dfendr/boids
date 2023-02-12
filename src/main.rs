use nannou::prelude::*;

const N_BOIDS: u32 = 300;

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
    size: f32,
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
        let size = (width * height) / 2.0;
        let position = vec2(x, y);
        let velocity = vec2(random_range(-1.0, 1.0), random_range(-1.0, 1.0));
        let acceleration = vec2(0.0, 0.0);
        let max_force = 0.1;
        let max_speed = 5.0;
        let min_speed = 3.0;
        let visual_range = 80.0;
        let protected_range = 20.0;
        Boid {
            width,
            height,
            size,
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
        let mut steering = Vec2::ZERO;
        if nearby_boids.is_empty() {
            return steering;
        }
        let mut too_close_count = 0;

        // Get Average Velocity
        for other in nearby_boids {
            if self.position.distance(other.position) < self.protected_range {
                steering = steering + self.position - other.position;
                too_close_count += 1;
            }
        }
        if too_close_count == 0 {
            return Vec2::ZERO;
        }
        steering /= too_close_count as f32;

        // Scale accordingly
        steering *= self.max_force;
        steering = steering.clamp_length_max(self.max_force * 2.0);

        steering
    }

    fn cohere(&self, nearby_boids: &[&Boid]) -> Vec2 {
        let mut steering = Vec2::ZERO;
        if nearby_boids.is_empty() {
            return steering;
        }
        let mut neighbour_count = 0;

        // Get Average Velocity
        for other in nearby_boids {
            if self.position.distance(other.position) > self.protected_range {
                steering = steering + other.position;
                neighbour_count += 1;
            }
        }
        if neighbour_count == 0 {
            return Vec2::ZERO;
        }
        steering /= neighbour_count as f32;
        steering -= self.position;
        steering = steering.clamp_length_max(self.max_speed);
        steering -= self.velocity;
        steering = limit_2d(steering, self.max_force);

        steering
    }

    fn apply_force(&mut self, force: Vec2) {
        // We could add mass here if we want A = F / M
        self.acceleration += force;
    }

    fn wrap(&mut self, win: &Rect) {
        let size = self.size;
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
            let steer = (desired - self.velocity).clamp_length_max(self.max_force * 3.0);
            self.apply_force(steer);
        }
    }

    fn nearby_boids<'a>(&self, flock: &'a Vec<Boid>) -> Vec<&'a Boid> {
        let mut nearby_boids: Vec<&Boid> = Vec::new();
        let mut close_boids: Vec<&Boid> = Vec::new();
        for other in flock {
            //(DotProduct(boid.velocity, otherboid.position - boid.position) > 0)
            let d = Vec2::distance(self.position, other.position).abs();
            // Check if other in front of self
            // let dot = self.position.dot(other.position - self.position) > 0.0;
            if other != self && d < self.visual_range {
                nearby_boids.push(other);
                if d < self.protected_range{
                    close_boids.push(other);
                }
            }
        }
        nearby_boids
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
        let nearby_boids = model.flock[i].nearby_boids(&model.flock);
        let v1 = model.flock[i].align(&nearby_boids);
        let v2 = model.flock[i].separate(&nearby_boids);
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
