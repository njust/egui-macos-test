#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release


use std::io::{BufReader, Cursor};
use anyhow::Result;
use eframe::{egui, Theme};
use hyper::http::version;
use log::{info, error, debug};
use poll_promise::Promise;
use update_notifier::UpdateNotifier;
mod update_notifier;
mod util;

pub fn scale_ui_with_keyboard_shortcuts(ctx: &egui::Context, native_pixels_per_point: Option<f32>) {
    // Using winit on Mac the key with the Plus sign on it is reported as the Equals key
    // (with both English and Swedish keyboard).)
    let zoom_in = egui::KeyboardShortcut::new(egui::Modifiers::COMMAND, egui::Key::ArrowUp);
    let zoom_out = egui::KeyboardShortcut::new(egui::Modifiers::COMMAND, egui::Key::ArrowDown);
    let reset = egui::KeyboardShortcut::new(egui::Modifiers::COMMAND, egui::Key::Num0);

    let zoom_in = ctx.input_mut(|input| input.consume_shortcut(&zoom_in));
    let zoom_out = ctx.input_mut(|input| input.consume_shortcut(&zoom_out));
    let reset = ctx.input_mut(|input| input.consume_shortcut(&reset));

    let mut pixels_per_point = ctx.pixels_per_point();

    if zoom_in {
        pixels_per_point += 0.1;
    }
    if zoom_out {
        pixels_per_point -= 0.1;
    }
    pixels_per_point = pixels_per_point.clamp(0.2, 5.);
    pixels_per_point = (pixels_per_point * 10.).round() / 10.;
    if reset {
        if let Some(native_pixels_per_point) = native_pixels_per_point {
            pixels_per_point = native_pixels_per_point;
        }
    }

    if pixels_per_point != ctx.pixels_per_point() {
        debug!("Changed GUI scale to {}", pixels_per_point);
        ctx.set_pixels_per_point(pixels_per_point);
    }
}
fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let update_check = Promise::spawn_thread("update_check", move || {
        info!("Background thread");
        let res = update_notifier::get_latest_version();
        info!("Request repaint");
        res
    });

    loop {
        if let Some(chk) = update_check.ready() {
            match chk {
                Ok(res) => {
                    info!("Ok: {:?}", res);
                    break;
                }
                Err(e) => {
                    error!("Failed: {}", e);
                    break;
                }
            }
        }
    }

    let icon_data = include_bytes!("../assets/icon/appIcon.png");
    let mut icon_data = BufReader::new(Cursor::new(icon_data));
    let app_img = image::load(&mut icon_data, image::ImageFormat::Png)
        .expect("Failed to load app icon")
        .to_rgba8();
    let (icon_width, icon_height) = app_img.dimensions();

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::Vec2::new(1600., 800.)),
        follow_system_theme: false,
        centered: true,
        icon_data: Some(eframe::IconData {
            width: icon_width,
            height: icon_height,
            rgba: app_img.into_raw(),
        }),
        default_theme: Theme::Light,
        ..eframe::NativeOptions::default()
    };

    // Our application state:
    let mut name = "Arthur".to_owned();
    let mut age = 42;
    let mut notifier = UpdateNotifier::new();

    info!("Before");
    let res = eframe::run_simple_native("My egui App", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut name)
                    .labelled_by(name_label.id);
            });
            ui.add(egui::Slider::new(&mut age, 0..=120).text("age"));
            if ui.button("Click each year").clicked() {
                age += 1;
            }
            ui.label(format!("Hello '{name}', age {age}"));
        });
        notifier.show(ctx);
    });
    info!("After");
    res
}