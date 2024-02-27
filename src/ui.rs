use nannou::prelude::*;

use crate::model::Model;

pub fn draw_information_text(app: &App, model: &Model, draw: &Draw) {
    draw_settings(app, model, draw);
    draw_instructions(app, model, draw);
}

pub fn draw_settings(app: &App, model: &Model, draw: &Draw) {
    let position = app
        .window_rect()
        .pad_bottom(50.0)
        .pad_left(100.0)
        .bottom_left();
    if model.show_text {
        draw.text(
            format!(
                "Boids: {}\nAlignment: {:.1}%\nCohesion: {:.1}%\nSeparation: {:.1}%\nCursor Mode: {:#?}\nFPS {:.0}",
                model.boid_options.flock_size,
                // Added 0.001 so -0.0 wouldn't show up when rounding
                (model.alignment_modifier + 0.0001) * 100.0,
                (model.cohesion_modifier + 0.0001) * 100.0,
                (model.separation_modifier + 0.0001) * 100.0,
                model.cursor_mode,
                app.fps()

            )
            .trim(),
        )
        .color(BLACK)
        .left_justify()
        .xy(position);
    }
}

pub fn draw_instructions(app: &App, model: &Model, draw: &Draw) {
    let position = app.window_rect().pad_top(50.0).pad_left(100.0).top_left();
    if model.show_text {
        draw.text(
            " Up/Down   Add/Remove Boids\nu/i   Alignment\nj/k   Cohesion\nm/,  Separation\nc     Cursor Mode\nh     Hide Text\nq   Quit"
                .to_string()
                .trim(),
        )
        .left_justify()
        .color(BLACK)
        .xy(position);
    }
}
