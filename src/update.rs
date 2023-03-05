use nannou::{
    prelude::{Signed, Update},
    App,
};
// use ui::update_ui;
use crate::model::Model;

pub fn update(app: &App, model: &mut Model, _update: Update) {
    update_model(app, model);
    if model.flock.is_empty() {
        return;
    };
    for i in 0..model.predators.len() {
        let close_predators = model.predators[i].close_predators(&model.predators);
        let (mut nearby_boids, close_boids) = model.predators[i].get_neighbours(&model.flock);
        nearby_boids.extend(close_boids);
        let separation = model.predators[i].separate(&close_predators);
        let hunting_force = model.predators[i].cohere(&nearby_boids);
        let bounds_force = model.predators[i].avoid_bounds(
            &app.window(model.main_window)
                .expect("Problem retrieving main window")
                .rect(),
        );

        model.predators[i].acceleration += hunting_force + bounds_force + separation;
        model.predators[i].update();
    }

    for i in 0..model.flock.len() {
        let (nearby_boids, close_boids) = model.flock[i].get_neighbours(&model.flock);
        let alignment = model.flock[i].align(&nearby_boids) * model.alignment_modifier;
        let seperation = model.flock[i].separate(&close_boids) * model.separation_modifier;
        let cohesion = model.flock[i].cohere(&nearby_boids) * model.cohesion_modifier;
        let predator_avoidance = model.flock[i].avoid_predators(&model.predators);
        let cursor_interaction = model.flock[i].cursor_interaction(app, &model.cursor_mode);
        let bounds_force = model.flock[i].avoid_bounds(
            &app.window(model.main_window)
                .expect("Problem retrieving main window")
                .rect(),
        );
        model.flock[i].acceleration += alignment
            + seperation
            + cohesion
            + bounds_force
            + predator_avoidance
            + cursor_interaction;

        model.flock[i].wrap(
            &app.window(model.main_window)
                .expect("Problem retrieving main window")
                .rect(),
        );
        model.flock[i].update();
    }
}

fn update_model(app: &App, model: &mut Model) {
    adjust_boid_count(app, model);
}

fn adjust_boid_count(app: &App, model: &mut Model) {
    if model.predator_options.n_mod == 0 && model.boid_options.n_mod == 0
        || model.predator_options.flock_size == 0 && model.boid_options.flock_size == 0
    {
        return;
    }

    let bounds = app
        .window(model.main_window)
        .expect("Main window should be running")
        .rect();

    // if n_mod isn't 0, and flock/predators isn't empty or 1000, increase by n_mod
    // then set n_mod to 0

    for _ in 0..model.predator_options.n_mod.abs() {
        match model.predator_options.n_mod.signum() {
            1 => {
                for _ in 0..model.predator_options.n_mod {
                    model.add_boid(bounds, model.predator_options.boid_type)
                }
            }
            -1 => model.remove_boid(bounds, model.predator_options.boid_type),
            0 => {}
            _ => panic!("Unreachable match condition"),
        };
    }
    model.predator_options.n_mod = 0;
    model.predator_options.flock_size = model.predators.len() as u32;

    for _ in 0..model.boid_options.n_mod.abs() {
        match model.boid_options.n_mod.signum() {
            1 => model.add_boid(bounds, model.boid_options.boid_type),
            -1 => model.remove_boid(bounds, model.boid_options.boid_type),
            0 => {}
            _ => panic!("Unreachable match condition"),
        };
    }
    model.boid_options.n_mod = 0;
    model.boid_options.flock_size = model.flock.len() as u32;
}
