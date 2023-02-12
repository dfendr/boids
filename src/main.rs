use nannou::prelude::*;
mod boids;
mod flock;
use boids::Boid;
use flock::Flock;
use nannou_egui::{egui, Egui};

const N_BOIDS: u32 = 300;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    main_window: WindowId,
    ui: Egui,
    flock: Vec<Boid>,
}

fn ui_view(_app: &App, model: &Model, frame: Frame) {
    model
        .ui
        .draw_to_frame(&frame)
        .expect("Error drawing UI Window")
}

fn raw_ui_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.ui.handle_raw_event(event);
}

fn update_ui(model: &mut Model) {
    let ctx = model.ui.begin_frame();
    egui::Window::new("Boids Options")
        .collapsible(false)
        .show(&ctx, |ui| {
            // ui.add(Slider::new(&mut model.disp_adj, 0.0..=5.0).text("Displacement"));
            // ui.add(Slider::new(&mut model.rot_adj, 0.0..=5.0).text("Rotation"));
            // ui.add(Slider::new(&mut model.motion, 0.0..=1.0).text("Motion"));
            if ui.add(egui::Button::new("Randomize")).clicked() {
                // model.random_seed = random_range(0, 1_000_000);
            }
        });
}

fn model(app: &App) -> Model {
    let main_window = app
        .new_window()
        .title(app.exe_name().unwrap_or_default())
        .view(view)
        .build()
        .expect("ah geez something happened...I'm sorry.");
    let flock = Flock::new_flock(app.window_rect(), N_BOIDS);

    let ui_window = app
        .new_window()
        .title(
            app.exe_name()
                .expect("Contact the police if this throws an error.")
                + " controls",
        )
        .size(280, 180)
        .view(ui_view)
        .raw_event(raw_ui_event)
        .key_pressed(key_pressed)
        .build()
        .expect("Error drawing UI Controls window");

    let ui = Egui::from_window(&app.window(ui_window).expect("Window ID Invalid -- Error"));

    Model {
        main_window,
        ui,
        flock,
    }
}

fn key_pressed(app: &App, model: &mut Model, key: Key) {
    match key {
        Key::S => {
            if let Some(window) = app.window(model.main_window) {
                window.capture_frame(app.exe_name().expect("Problem saving image") + ".png")
            }
        }
        //     Key::Up => {
        //         // model.disp_adj += 0.1;
        //     }
        //     Key::Down => {
        //         // if model.disp_adj >= 0.0 {
        //         //     model.disp_adj -= 0.1;
        //         }
        //     }
        //     Key::Left => {
        //         // model.rot_adj += 0.1;
        //     }
        //     Key::Right => {
        //         // if model.rot_adj >= 0.0 {
        //         //     model.rot_adj -= 0.1;
        //         // }
        //     }
        _other_key => {}
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    update_ui(model);
    for i in 0..model.flock.len() {
        let (nearby_boids, close_boids) = model.flock[i].get_neighbours(&model.flock);
        let alignment = model.flock[i].align(&nearby_boids);
        let seperation = model.flock[i].separate(&close_boids);
        let cohesion = model.flock[i].cohere(&nearby_boids);
        model.flock[i].acceleration += alignment + seperation + cohesion;

        model.flock[i].avoid_bounds(
            &app.window(model.main_window)
                .expect("Problem retrieving main window")
                .rect(),
        );
        model.flock[i].wrap(
            &app.window(model.main_window)
                .expect("Problem retrieving main window")
                .rect(),
        );
        model.flock[i].update();
    }
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
