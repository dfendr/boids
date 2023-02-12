use model::model;
use model::Model;
use nannou::prelude::*;
mod boids;
mod flock;
mod keymaps;
mod model;
mod ui;
mod update;
use flock::Flock;
use ui::draw_text;
use update::update;

fn main() {
    nannou::app(model).update(update).run();
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(SKYBLUE);
    draw_text(app, model, &draw);

    for boid in &model.flock {
        boid.show(&draw);
    }

    draw.to_frame(app, &frame)
        .expect("WHOA NELLY THIS AINT GOOD");
}
