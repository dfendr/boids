use nannou::prelude::*;
use nannou_egui::egui::{self, Slider};

use crate::model::Model;

pub fn ui_view(_app: &App, model: &Model, frame: Frame) {
    model
        .ui
        .draw_to_frame(&frame)
        .expect("Error drawing UI Window")
}

pub fn raw_ui_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.ui.handle_raw_event(event);
}

pub fn update_ui(model: &mut Model) {
    let ctx = model.ui.begin_frame();
    egui::Window::new("Boids Options")
        .collapsible(false)
        .show(&ctx, |ui| {
            ui.add(
                Slider::new(&mut model.n_boids, 1..=2000)
                    .text("Number of Boids")
                    .show_value(true),
            );
            ui.add(
                Slider::new(&mut model.alignment_modifier, -2.0..=2.0)
                    .text("Alignment")
                    .show_value(true),
            );
            ui.add(
                Slider::new(&mut model.cohesion_modifier, -2.0..=2.0)
                    .text("Cohesion")
                    .show_value(true),
            );
            ui.add(
                Slider::new(&mut model.separation_modifier, -2.0..=2.0)
                    .text("Separation")
                    .show_value(true),
            );
            if ui.add(egui::Button::new("Reset")).clicked() {
                // model.random_seed = random_range(0, 1_000_000);
            }
        });
}
