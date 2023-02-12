use crate::{
    boids::Boid,
    keymaps::key_pressed,
    // ui::{raw_ui_event, ui_view},
    view,
    Flock,
};

use nannou::prelude::*;

pub struct Model {
    pub main_window: WindowId,
    // pub ui: Egui,
    pub show_text: bool,
    pub flock: Vec<Boid>,
    pub n_boids: i32,
    pub boid_height: f32,
    pub boid_width: f32,
    pub boid_starting_velocity: f32,
    pub alignment_modifier: f32,
    pub cohesion_modifier: f32,
    pub separation_modifier: f32,
    pub boid_min_speed: f32,
    pub boid_max_speed: f32,
    pub boid_visual_range: f32,
    pub boid_separation_range: f32,
    pub max_force: f32,
}

impl Model {
    pub fn add_boid(&mut self, bounds: Rect) {
        let (left, right, bottom, top) = bounds.l_r_b_t();
        let boid = Boid::new(random_range(left, right), random_range(bottom, top));
        self.flock.push(boid);
    }
    // pub fn add_boid_center(&mut self) {
    //     let boid = Boid::new(0.0, 0.0);
    //     self.flock.push(boid);
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
    let n_boids = 100;
    let alignment_modifier = 1.0;
    let cohesion_modifier = 1.0;
    let separation_modifier = 1.0;
    let boid_width = 10.0;
    let boid_height = 20.0;
    let boid_starting_velocity = 1.0;
    let boid_max_speed = 4.0;
    let boid_min_speed = 2.0;
    let boid_visual_range = 100.0;
    let boid_separation_range = 30.0;
    let max_force = 0.2;

    let flock = Flock::new_flock(app.window_rect(), n_boids as u32);

    // let ui_window = app
    //     .new_window()
    //     .title(
    //         app.exe_name()
    //             .expect("contact the police if this throws an error.")
    //             + " controls",
    //     )
    //     .size(280, 170)
    //     .view(ui_view)
    //     .raw_event(raw_ui_event)
    //     .build()
    //     .expect("error drawing ui controls window");
    //
    // let ui = Egui::from_window(&app.window(ui_window).expect("window id invalid -- error"));

    Model {
        main_window,
        // ui,
        show_text,
        flock,
        n_boids,
        alignment_modifier,
        cohesion_modifier,
        separation_modifier,
        boid_height,
        boid_width,
        boid_starting_velocity,
        boid_min_speed,
        boid_max_speed,
        boid_visual_range,
        boid_separation_range,
        max_force,
    }
}
