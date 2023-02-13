use crate::{
    boids::{
        Boid,
        BoidType::{Predator, Prey},
    },
    cursor::CursorMode,
    keymaps::key_pressed,
    view, Flock,
};

use nannou::prelude::*;

pub struct Model {
    pub main_window: WindowId,
    // pub ui: Egui,
    pub cursor_mode: CursorMode,
    pub show_text: bool,
    pub flock: Vec<Boid>,
    pub predators: Vec<Boid>,
    pub ticks: u64,
    pub n_boids: i32,
    pub boid_starting_velocity: f32,
    pub alignment_modifier: f32,
    pub cohesion_modifier: f32,
    pub separation_modifier: f32,
    pub boid_min_speed: f32,
    pub boid_max_speed: f32,
    pub max_force: f32,
}

impl Model {
    pub fn add_boid(&mut self, bounds: Rect) {
        let (left, right, bottom, top) = bounds.l_r_b_t();
        let boid = Boid::new(random_range(left, right), random_range(bottom, top), Prey);
        self.flock.push(boid);
    }
    // pub fn add_predator(&mut self, bounds: Rect) {
    //     let (left, right, bottom, top) = bounds.l_r_b_t();
    //     let boid = Boid::new(
    //         random_range(left, right),
    //         random_range(bottom, top),
    //         Predator,
    //     );
    //     self.predators.push(boid);
    // }
}

pub fn model(app: &App) -> Model {
    let main_window = app
        .new_window()
        .title(app.exe_name().unwrap_or_default())
        .view(view)
        .key_pressed(key_pressed)
        .build()
        .expect("ah geez something happened...i'm sorry.");

    let show_text = true;
    // defaults
    let cursor_mode = CursorMode::Ignore;
    let ticks: u64 = 0;
    let n_boids = 100;
    let alignment_modifier = 0.25;
    let cohesion_modifier = 0.25;
    let separation_modifier = 0.75;
    let boid_starting_velocity = 1.0;
    let boid_max_speed = 3.0;
    let boid_min_speed = 1.0;
    let max_force = 0.2;

    let flock = Flock::new_flock(app.window_rect(), n_boids as u32, Prey);
    let predators = Flock::new_flock(app.window_rect(), 1, Predator);

    Model {
        main_window,
        // ui,
        cursor_mode,
        show_text,
        ticks,
        flock,
        predators,
        n_boids,
        alignment_modifier,
        cohesion_modifier,
        separation_modifier,
        boid_starting_velocity,
        boid_min_speed,
        boid_max_speed,
        max_force,
    }
}
