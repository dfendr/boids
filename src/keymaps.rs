use std::process::exit;

use nannou::{prelude::Key, App};

use crate::model::Model;

pub fn key_pressed(app: &App, model: &mut Model, key: Key) {
    match key {
        Key::S => {
            if let Some(window) = app.window(model.main_window) {
                window.capture_frame(app.exe_name().expect("Problem saving image") + ".png");
            }
        }
        Key::U => {
            model.alignment_modifier -= 0.05;
        }
        Key::C => {
            model.cursor_mode = model.cursor_mode.next();
        }
        Key::T => {
            model.theme = model.theme.next();
        }

        Key::I => {
            model.alignment_modifier += 0.05;
        }
        Key::J => {
            model.cohesion_modifier -= 0.05;
        }
        Key::K => {
            model.cohesion_modifier += 0.05;
        }
        Key::M => {
            model.separation_modifier -= 0.05;
        }
        Key::Comma => {
            model.separation_modifier += 0.05;
        }
        Key::Up => {
            if model.boid_options.flock_size >= 1000 {
                return;
            }
            if model.boid_options.flock_size < 5 {
                model.boid_options.n_mod += 1;
            } else {
                model.boid_options.n_mod += 5;
            };
        }
        Key::Down => {
            if model.boid_options.flock_size == 0 {
                return;
            }
            if model.boid_options.flock_size > 5 {
                model.boid_options.n_mod -= 5;
            } else if model.boid_options.flock_size > 1 {
                model.boid_options.n_mod -= 1;
            };
        }
        Key::Left => model.predator_options.n_mod -= 1,
        Key::Right => {
            if model.predator_options.flock_size > 5 {
                return;
            }
            model.predator_options.n_mod += 1;
        }
        Key::H => {
            model.show_text = !model.show_text;
        }
        Key::Q => {
            exit(1);
        }
        _other_key => {}
    }
}
