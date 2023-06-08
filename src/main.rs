#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

pub mod app;
mod eval;
mod plotter;
mod ui;

use app::GraphWar;
use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        min_window_size: Some(egui::vec2(500.0, 500.0)),
        maximized: true,
        centered: true,
        app_id: Some("graphwar".to_string()),
        ..Default::default()
    };
    eframe::run_native(
        "Graphwar",
        options,
        Box::new(|_cc| Box::<GraphWar>::default()),
    )
}
