use crate::{
    boids::{
        Boid,
        BoidType::{self, Predator, Prey},
    },
    cursor::CursorMode,
    keymaps::key_pressed,
    theme::Theme,
    view, Flock,
};

use nannou::prelude::*;

pub struct BoidOptions {
    pub boid_type: BoidType,
    pub starting_velocity: f32,
    pub colour: Rgb8,
    pub min_speed: f32,
    pub max_speed: f32,
    pub flock_size: usize,
    pub n_mod: i32,
}

pub struct Model {
    pub main_window: WindowId,
    pub boid_options: BoidOptions,
    pub predator_options: BoidOptions,
    pub cursor_mode: CursorMode,
    pub show_text: bool,
    pub theme: Theme,
    pub flock: Vec<Boid>,
    pub predators: Vec<Boid>,
    pub alignment_modifier: f32,
    pub cohesion_modifier: f32,
    pub separation_modifier: f32,
    pub max_force: f32,
}

impl Model {
    pub fn add_boid(&mut self, bounds: Rect, boid_type: BoidType) {
        let (left, right, bottom, top) = bounds.l_r_b_t();
        let boid = Boid::new(
            random_range(left, right),
            random_range(bottom, top),
            boid_type,
        );

        match boid_type {
            Prey => self.flock.push(boid),
            Predator => self.predators.push(boid),
        }
    }

    pub fn remove_boid(&mut self, boid_type: BoidType) {
        match boid_type {
            Prey => self.flock.pop(),
            Predator => self.predators.pop(),
        };
    }
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
    let theme = Theme::Normal;
    let alignment_modifier = 0.25;
    let cohesion_modifier = 0.25;
    let separation_modifier = 0.75;
    let boid_options = BoidOptions {
        boid_type: BoidType::Prey,
        starting_velocity: 1.0,
        colour: BLACK,
        min_speed: 1.0,
        max_speed: 3.0,
        flock_size: 100,
        n_mod: 0,
    };
    let predator_options = BoidOptions {
        boid_type: BoidType::Predator,
        starting_velocity: 1.5,
        colour: RED,
        min_speed: 1.0,
        max_speed: 3.5,
        flock_size: 1,
        n_mod: 0,
    };
    let max_force = 0.2;

    let flock = Flock::new_flock(app.window_rect(), boid_options.flock_size, Prey);
    let predators = Flock::new_flock(app.window_rect(), 1, Predator);

    Model {
        main_window,
        boid_options,
        predator_options,
        cursor_mode,
        show_text,
        theme,
        flock,
        predators,
        alignment_modifier,
        cohesion_modifier,
        separation_modifier,
        max_force,
    }
}
