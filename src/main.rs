use model::model;
use model::Model;
use nannou::prelude::*;
mod boids;
mod cursor;
mod flock;
mod keymaps;
mod model;
mod ui;
mod update;
use flock::Flock;
use ui::draw_information_text;
use update::update;


fn main() {
    nannou::app(model)
        .update(update)
        .run();
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    app.window(model.main_window)
        .expect("Error retrieving main window")
        .set_cursor_visible(false);

    draw.background().color(SKYBLUE);
    draw_information_text(app, model, &draw);
    for boid in &model.flock {
        boid.show(&draw);
    }
    draw.ellipse()
        .xy(app.mouse.position())
        .height(5.0)
        .width(5.0);

    draw.to_frame(app, &frame)
        .expect("WHOA NELLY THIS AINT GOOD");
}
