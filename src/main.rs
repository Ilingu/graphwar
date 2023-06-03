#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::f64::consts::PI;

use eframe::egui;
use egui::{
    plot::{Line, Plot, PlotPoints, Polygon},
    Color32,
};

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(500.0, 500.0)),
        resizable: false,
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

const PLOT_RESOLUTION: isize = 100;

struct GraphWar {}

impl Default for GraphWar {
    fn default() -> Self {
        Self {}
    }
}

impl eframe::App for GraphWar {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let plot = Plot::new("measurements");
            plot.show(ui, |plot_ui| {
                // this is y=x, on [-2,2]
                plot_ui.line(Line::new(PlotPoints::new(
                    ((-2 * PLOT_RESOLUTION)..=(2 * PLOT_RESOLUTION))
                        .map(|x| x as f64 / PLOT_RESOLUTION as f64)
                        .map(|x| [x, x])
                        .collect(),
                )));

                // this is y=-x, on [-2,2]
                plot_ui.line(Line::new(PlotPoints::new(
                    ((-2 * PLOT_RESOLUTION)..=(2 * PLOT_RESOLUTION))
                        .map(|x| x as f64 / PLOT_RESOLUTION as f64)
                        .map(|x| [x, -x])
                        .collect(),
                )));

                // This is a circle
                plot_ui.polygon(
                    Polygon::new(PlotPoints::new(
                        (0..1000)
                            .map(|k| k as f64)
                            .map(|k| 2.0 * k * PI / 1000.0) // racine nième de l'unité
                            .map(|x| [x.cos(), x.sin()])
                            .collect(),
                    ))
                    .color(Color32::from_rgb(255, 0, 0)),
                );
            });
        });
    }
}
