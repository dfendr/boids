use nannou::prelude::*;

use crate::model::Model;

pub fn draw_text(app: &App, model: &Model, draw: &Draw) {
    draw_settings(app, model, draw);
    draw_instructions(app, model, draw);
}

pub fn draw_settings(_app: &App, model: &Model, draw: &Draw) {
    let position = _app
        .window_rect()
        .pad_bottom(50.0)
        .pad_left(100.0)
        .bottom_left();
    if model.show_text{
        draw.text(
            format!(
                "Boids: {}\nAlignment: {:.1}%\nCohesion: {:.1}%\nSeparation: {:.1}%",
                model.n_boids,
                (model.alignment_modifier + 0.0001) * 100.0,
                (model.cohesion_modifier + 0.0001) * 100.0,
                (model.separation_modifier + 0.0001) * 100.0,
            )
            .trim(),
        )
        .color(BLACK)
        .left_justify()
        .xy(position);
    }
}

pub fn draw_instructions(_app: &App, model: &Model, draw: &Draw) {
    let position = _app.window_rect().pad_top(50.0).pad_left(100.0).top_left();
    if model.show_text{
        draw.text(
            "Add/Remove: <Up/Down>\nAlignment: <u/i>\nCohesion: <j/k>\nSeparation: <m/,>\nHide Text: <h>\nQuit: <q>"
                .to_string()
                .trim(),
        )
        .left_justify()
        .color(BLACK)
        .xy(position);
    }
}

// pub fn ui_view(_app: &App, model: &Model, frame: Frame) {
//     model
//         .ui
//         .draw_to_frame(&frame)
//         .expect("Error drawing UI Window")
// }
//
// pub fn raw_ui_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
//     model.ui.handle_raw_event(event);
// }
//
// pub fn update_ui(model: &mut Model) {
//     let ctx = model.ui.begin_frame();
//     egui::Window::new("Boids Options")
//         .collapsible(false)
//         .show(&ctx, |ui| {
//             ui.add(
//                 Slider::new(&mut model.n_boids, 1..=2000)
//                     .text("Number of Boids")
//                     .show_value(true),
//             );
//             ui.add(
//                 Slider::new(&mut model.alignment_modifier, -2.0..=2.0)
//                     .text("Alignment")
//                     .show_value(true),
//             );
//             ui.add(
//                 Slider::new(&mut model.cohesion_modifier, -2.0..=2.0)
//                     .text("Cohesion")
//                     .show_value(true),
//             );
//             ui.add(
//                 Slider::new(&mut model.separation_modifier, -2.0..=2.0)
//                     .text("Separation")
//                     .show_value(true),
//             );
//             if ui.add(egui::Button::new("Reset")).clicked() {
//                 // model.random_seed = random_range(0, 1_000_000);
//             }
//         });
// }
