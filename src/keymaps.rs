use std::process::exit;

use nannou::{prelude::Key, App};

use crate::model::Model;

pub fn key_pressed(app: &App, model: &mut Model, key: Key) {
    match key {
        Key::S => {
            if let Some(window) = app.window(model.main_window) {
                window.capture_frame(app.exe_name().expect("Problem saving image") + ".png")
            }
        }
        Key::U => {
            model.alignment_modifier -= 0.05;
        }
        Key::C => {
            model.cursor_mode = model.cursor_mode.next();
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
            if model.n_boids == 1000 {
                return;
            }
            if model.n_boids < 5 {
                model.n_boids += 1
            } else {
                model.n_boids += 5
            };
        }
        Key::Down => {
            if model.n_boids <= 5 && model.n_boids > 1 {
                model.n_boids -= 1
            }
            if model.n_boids > 6 {
                model.n_boids -= 5;
            }
        }
        Key::Left => {
            // TODO: Add predators
            // model.n_predators += 1;
        }
        Key::Right => {
            // TODO: Add predators
            // model.n_predators -= 1;
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
