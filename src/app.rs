use std::time::Duration;

use egui::plot::PlotPoint;
use egui_extras::RetainedImage;

use rand::Rng;

use crate::{
    eval::MathExpression,
    plotter::{get_app_plot, get_graph_plot_points, Plotter},
    ui::{load_image, rich_text, Message, UITypes},
};

pub struct GraphWar {
    equation: String,

    graph_cached_points: Option<Vec<PlotPoint>>,
    graph_resolution: usize,

    player: PlotPoint,
    ennemies: Vec<PlotPoint>,

    player_texture: RetainedImage,
    ennemy_texture: RetainedImage,

    messages: Vec<Message>,
}

impl GraphWar {
    fn build_graph(&mut self) {
        match MathExpression::new(&self.equation) {
            Ok(math_expr) => {
                self.graph_cached_points = Some(
                    get_graph_plot_points(&math_expr, (-25, 25), self.graph_resolution)
                        .points()
                        .to_vec(),
                );
            }
            Err(_) => {
                self.messages.push(Message::new(
                    "unevaluable mathematical expression".to_string(),
                    Duration::from_secs(4),
                    UITypes::Error,
                ));
            }
        }
    }

    fn hide_graph(&mut self) {
        self.graph_cached_points = None
    }

    fn spawn_entity() -> PlotPoint {
        let mut rng = rand::thread_rng();
        let (x, y) = (
            rng.gen_range(-25..=25) as f64,
            rng.gen_range(-25..=25) as f64,
        );
        PlotPoint { x, y }
    }
}

impl Default for GraphWar {
    fn default() -> Self {
        let enemies_nums = rand::thread_rng().gen_range(1..=5);

        let image_player_data = include_bytes!("../assets/player.png");
        let image_ennemy_data = include_bytes!("../assets/ennemy.png");

        Self {
            equation: String::new(),

            graph_resolution: 100,
            graph_cached_points: None,

            player: Self::spawn_entity(),
            ennemies: (0..enemies_nums).map(|_| Self::spawn_entity()).collect(),

            player_texture: RetainedImage::from_color_image(
                "player.png",
                load_image(image_player_data).unwrap(),
            ),
            ennemy_texture: RetainedImage::from_color_image(
                "ennemy.png",
                load_image(image_ennemy_data).unwrap(),
            ),

            messages: vec![],
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

            // PLOT
            ui.vertical_centered_justified(|ui| {
                let plot = get_app_plot().height(
                    min_square_size
                        - if is_messages {
                            100.0 + 25.0 * self.messages.len() as f32
                        } else {
                            100.0
                        },
                );

                plot.show(ui, |plot_ui| {
                    if let Some(points) = &self.graph_cached_points {
                        plot_ui.render_graph(points);
                    }
                    plot_ui.render_player_image(self.player, &self.player_texture, ctx)
                });
            });

            // Button, Input and Messages
            ui.vertical_centered_justified(|ui| {
                let name_label = ui.label(rich_text("Line equation:", UITypes::Neutral));
                let equation_text_ui = ui
                    .text_edit_singleline(&mut self.equation)
                    .labelled_by(name_label.id);
                if equation_text_ui.changed() {
                    self.hide_graph();
                }
                if equation_text_ui.lost_focus() {
                    self.build_graph();
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
                    self.build_graph();
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
