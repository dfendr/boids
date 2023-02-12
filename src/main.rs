use model::Model;
use nannou::prelude::*;
mod boids;
mod keymaps;
mod flock;
mod model;
pub mod ui;
mod update;
use flock::Flock;
use update::update;

fn main() {
    nannou::app(model::model).update(update).run();
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(SKYBLUE);

    for boid in &model.flock {
        boid.show(&draw);
    }

    draw.to_frame(app, &frame)
        .expect("WHOA NELLY THIS AINT GOOD");
}
