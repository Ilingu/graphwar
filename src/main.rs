#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod eval;
mod plotter;

use eframe::egui;
use egui::plot::Plot;
use eval::MathExpression;
use plotter::Plotter;

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

struct GraphWar {
    equation: String,
    graph_resolution: usize,
    show_eq: bool,
    error_message: String,
}

impl Default for GraphWar {
    fn default() -> Self {
        Self {
            graph_resolution: 100,
            equation: String::new(),
            error_message: String::new(),
            show_eq: false,
        }
    }
}

impl eframe::App for GraphWar {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let plot = Plot::new("game_graph").width(500.0).height(500.0);
            plot.show(ui, |plot_ui| {
                if self.show_eq {
                    match MathExpression::new(&self.equation) {
                        Ok(math_expr) => plot_ui.render_graph(&math_expr, self.graph_resolution),
                        Err(_) => {
                            self.show_eq = false;
                            self.error_message = "unevaluable mathematical expression".to_string();
                        }
                    }
                }
            });

            ui.horizontal(|ui| {
                let name_label = ui.label("Your line equation: ");
                let equation_text_ui = ui
                    .text_edit_multiline(&mut self.equation)
                    .labelled_by(name_label.id);
                if equation_text_ui.changed() {
                    self.show_eq = false;
                }
            });
            ui.add(
                egui::Slider::new(&mut self.graph_resolution, 1..=1000).text("Graph Resolution"),
            );
            if ui
                .button(if self.show_eq {
                    "Hide function"
                } else {
                    "Plot function"
                })
                .clicked()
            {
                self.show_eq = !self.show_eq;
            }
        });
    }
}
