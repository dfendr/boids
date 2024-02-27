use model::model;
use model::Model;
use nannou::prelude::*;
mod boids;
mod cursor;
mod flock;
mod keymaps;
mod model;
mod theme;
mod ui;
mod update;
use flock::Flock;
use theme::Theme;
use ui::draw_information_text;
use update::update;

fn main() {
    nannou::app(model).update(update).run();
}
// needless_pass_by_value added as Nannou framework expects Frame, not &Frame
#[allow(clippy::needless_pass_by_value)]
fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    app.window(model.main_window)
        .expect("Error retrieving main window")
        .set_cursor_visible(false);

    let background_color = match model.theme {
        Theme::Normal => SKYBLUE,
        Theme::Grey => GREY,
        Theme::DeepSea => BLACK,
    };

    draw.background().color(background_color);
    draw_information_text(app, model, &draw);
    for predator in &model.predators {
        predator.show(&draw, model);
    }
    for boid in &model.flock {
        boid.show(&draw, model);
    }
    draw.ellipse()
        .xy(app.mouse.position())
        .stroke_color(BLACK)
        .color(BLACK)
        .height(5.0)
        .width(5.0);

    draw.to_frame(app, &frame)
        .expect("WHOA NELLY THIS AINT GOOD");
}
