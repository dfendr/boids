use nannou::{prelude::Update, App};
// use ui::update_ui;
use crate::model::Model;

pub fn update(app: &App, model: &mut Model, _update: Update) {
    update_model(app, model);
    if model.flock.is_empty() {
        return;
    };
    for i in 0..model.predators.len() {
        let (mut nearby_boids, close_boids) = model.predators[i].get_neighbours(&model.flock);
        nearby_boids.extend(close_boids);
        model.predators[i].neighbour_count = nearby_boids.len();
        let hunting_force = model.predators[i].cohere(&nearby_boids);
        let bounds_force = model.predators[i].avoid_bounds(
            &app.window(model.main_window)
                .expect("Problem retrieving main window")
                .rect(),
        );

        model.predators[i].acceleration += hunting_force + bounds_force;
        model.predators[i].update();
    }

    for i in 0..model.flock.len() {
        let (nearby_boids, close_boids) = model.flock[i].get_neighbours(&model.flock);
        let alignment = model.flock[i].align(&nearby_boids) * model.alignment_modifier;
        let seperation = model.flock[i].separate(&close_boids) * model.separation_modifier;
        let cohesion = model.flock[i].cohere(&nearby_boids) * model.cohesion_modifier;
        let predator_avoidance = model.flock[i].avoid_predators(&model.predators);
        let cursor_interaction = model.flock[i].cursor_interaction(app, &model.cursor_mode);
        model.flock[i].neighbour_count = nearby_boids.len();
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
        // let delta_time = app.duration.since_prev_update.as_secs_f32();
        // println!("{delta_time}");
        model.flock[i].update();
    }
}

fn update_model(app: &App, model: &mut Model) {
    adjust_boid_count(app, model);
}

fn adjust_boid_count(app: &App, model: &mut Model) {
    if model.flock.is_empty() {
        return;
    }
    let bird_modifier = model.n_boids as isize - model.flock.len() as isize;

    if bird_modifier == 0 {
        return;
    }

    let bounds = app
        .window(model.main_window)
        .expect("Main window should be running")
        .rect();

    // Adjust birdcount by diffference
    let boid_count_difference = bird_modifier.unsigned_abs();
    match bird_modifier.cmp(&0) {
        std::cmp::Ordering::Greater => {
            for _ in 0..boid_count_difference {
                model.add_boid(bounds);
            }
        }
        std::cmp::Ordering::Less => {
            for _ in 0..boid_count_difference {
                model.flock.pop();
            }
        }
        std::cmp::Ordering::Equal => {}
    }
}
