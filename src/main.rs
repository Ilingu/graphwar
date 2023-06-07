#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod eval;
mod plotter;
mod ui;

use std::time::Duration;

use eframe::egui;
use egui::plot::Plot;
use eval::MathExpression;
use plotter::Plotter;
use ui::{rich_text, Message, UITypes};

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
    messages: Vec<Message>,
}

impl Default for GraphWar {
    fn default() -> Self {
        Self {
            graph_resolution: 100,
            equation: String::new(),
            messages: vec![],
            show_eq: false,
        }
    }
}

impl eframe::App for GraphWar {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let is_messages = !self.messages.is_empty();
        if is_messages {
            self.messages.retain(|msg| !msg.is_expired())
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            let window_size = ui.available_size();
            let min_square_size = window_size.x.min(window_size.y);

            ui.vertical_centered_justified(|ui| {
                let plot = Plot::new("game_graph")
                    .height(min_square_size - if is_messages { 125.0 } else { 100.0 }) // -x to be able to display ui bellow the plot
                    // block all plot camera movements
                    .allow_zoom(false)
                    .allow_drag(false)
                    .allow_boxed_zoom(false)
                    .allow_scroll(false)
                    // block plot bounds to [-25, 25] in the x and y axes
                    .include_x(-25.0)
                    .include_x(25.0)
                    .include_y(-25.0)
                    .include_y(25.0);
                plot.show(ui, |plot_ui| {
                    if self.show_eq {
                        match MathExpression::new(&self.equation) {
                            Ok(math_expr) => {
                                plot_ui.render_graph(&math_expr, self.graph_resolution)
                            }
                            Err(_) => {
                                self.show_eq = false;
                                self.messages.push(Message::new(
                                    "unevaluable mathematical expression".to_string(),
                                    Duration::from_secs(4),
                                    UITypes::Error,
                                ));
                            }
                        }
                    }
                });
            });

            ui.vertical_centered_justified(|ui| {
                let name_label = ui.label(rich_text("Line equation:", UITypes::Neutral));
                let equation_text_ui = ui
                    .text_edit_singleline(&mut self.equation)
                    .labelled_by(name_label.id);
                if equation_text_ui.changed() {
                    self.show_eq = false;
                }
                if equation_text_ui.lost_focus() {
                    self.show_eq = true;
                }
                /* graph_resolution is automatic
                ui.add(
                    egui::Slider::new(&mut self.graph_resolution, 1..=100).text("Graph Resolution"),
                );
                */
                ui.add_space(5.0);
                if ui
                    .button(rich_text("Shoot! 🎯", UITypes::Neutral))
                    .clicked()
                {
                    self.show_eq = true;
                }

                if is_messages {
                    for msg in &self.messages {
                        ui.add_space(5.0);
                        ui.label(msg.render());
                    }
                }
            })
        });
    }
}
