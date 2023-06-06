#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use anyhow::Result;
use log::info;
use poll_promise::Promise;

fn main() -> Result<()> {
    env_logger::init(); 
    let update_check = Promise::spawn_thread("update_check", move || {
        info!("Background thread");
        "test".to_string()
    });

    loop {
        info!("Check ready state");
        if let Some(chk) = update_check.ready() {
            info!("Res: {}", chk);
            break;
        }
    }
    Ok(())
}
