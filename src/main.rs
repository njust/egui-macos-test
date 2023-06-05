#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release


use std::io::{BufReader, Cursor};
use anyhow::Result;
use eframe::{egui, Theme};
use log::{info, error};

fn get_latest_version() -> Result<String> {
    let url = if cfg!(debug_assertions) {
        "http://localhost:1111/static/version.toml"
    } else {
        "https://kubelog.de/static/version.toml"
    };
    let version_info = reqwest::blocking::get(url)?.text()?;
    Ok(version_info)
}
fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    match get_latest_version() {
        Ok(version) => {
            info!("Version: {}", version);
        }
        Err(e) => {
            error!("Failed: {}", e);
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

    eframe::run_simple_native("My egui App", options, move |ctx, _frame| {
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
    })
}