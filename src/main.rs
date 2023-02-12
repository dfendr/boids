use nannou::prelude::*;
mod boids;
mod flock;
use boids::Boid;
use flock::Flock;

const N_BOIDS: u32 = 300;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
    flock: Vec<Boid>,
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
        let (nearby_boids, close_boids) = model.flock[i].get_neighbours(&model.flock);
        let alignment = model.flock[i].align(&nearby_boids);
        let seperation = model.flock[i].separate(&close_boids);
        let cohesion = model.flock[i].cohere(&nearby_boids);
        model.flock[i].acceleration += alignment + seperation + cohesion;
        model.flock[i].avoid_bounds(&app.window_rect());
        model.flock[i].wrap(&app.window_rect());
        model.flock[i].update();
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(SNOW);

    for boid in &model.flock {
        boid.show(&draw);
    }
    draw.to_frame(app, &frame)
        .expect("WHOA NELLY THIS AINT GOOD");
}
