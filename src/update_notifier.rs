use anyhow::Result;
use eframe::egui;
use log::info;
use poll_promise::Promise;
use serde::Deserialize;
use eframe::egui::{Align, Layout};

pub struct UpdateNotifier {
    update_check: Option<Promise<Result<VersionInfo>>>,
    open: bool,
}
#[derive(Default, Deserialize)]
struct VersionInfo {
    current: String,
    msg: Option<String>,
}

impl UpdateNotifier {
    pub fn new() -> Self {
        Self {
            update_check: None,
            open: true,
        }
    }

    pub fn show(&mut self, ctx: &egui::Context) {
        if let Some(check) = &self.update_check {
            if let Some(version) = check.ready() {
                if let Ok(version) = version {
                    {
                    }
                    let current_version = env!("CARGO_PKG_VERSION");
                    if version.current != current_version {
                        info!("Show update notify");
                        let (width, height, pos) = crate::util::get_wnd_center_pos(ctx, 300., 80.);
                        let mut close = false;
                        egui::Window::new("New version available")
                            .open(&mut self.open)
                            .default_pos(pos)
                            .default_width(width)
                            .default_height(height)
                            .show(ctx, |ui| {
                                ui.allocate_space([width, 2.].into());
                                ui.vertical_centered(|ui| {
                                    if let Some(msg) = &version.msg {
                                        ui.label(msg);
                                    }

                                    if ui.link(format!("Download version: {}", version.current)).clicked() {
                                        ctx.output_mut(|ctx| {
                                            ctx.open_url("https://kubelog.de/download/")
                                        })
                                    }
                                    ui.small(format!("(Your current version is: {})", current_version));
                                    ui.add_space(5.);
                                });


                                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                    if ui.button("Skip this version").clicked() {
                                        close = true;
                                    }

                                    if ui.button("Remind me later").clicked() {
                                        close = true;
                                    }
                                });
                            });

                        if close {
                            self.open = false;
                        }
                    }
                }
            }
        } else {
            let ctx = ctx.clone();
            info!("Spawn thread");
            self.update_check = Some(Promise::spawn_thread("update_check", move || {
                info!("Background thread");
                let res = Self::get_latest_version();
                ctx.request_repaint();
                info!("Request repaint");
                res
            }));
        }
    }

    fn get_latest_version() -> Result<VersionInfo> {
        let url = if cfg!(debug_assertions) {
            "https://kubelog.de/static/version.toml"
            // "http://localhost:1111/static/version.toml"
        } else {
            "https://kubelog.de/static/version.toml"
        };
        let version_info = reqwest::blocking::get(url)?.text()?;
        Ok(toml::de::from_str(&version_info)?)
    }
}
