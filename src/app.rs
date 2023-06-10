use std::time::Duration;

use egui::plot::{PlotPoint, PlotPoints};

use rand::Rng;

use crate::{
    eval::MathExpression,
    plotter::{compute_line_points, compute_polygon_points, get_app_plot, Plotter},
    ui::{rich_text, Message, UITypes},
};

struct EntitiesPos {
    obstacles: Vec<Vec<PlotPoint>>,
    player: (Vec<PlotPoint>, PlotPoint),
    enemies: Vec<Vec<PlotPoint>>,
}

pub struct GraphWar {
    equation: String,

    graph_cached_points: Option<Vec<PlotPoint>>,
    graph_animation_frame: usize,
    graph_resolution: usize,

    player: (Vec<PlotPoint>, PlotPoint),
    enemies: Vec<Vec<PlotPoint>>,
    obstacles: Vec<Vec<PlotPoint>>,

    messages: Vec<Message>,
}

impl Default for GraphWar {
    fn default() -> Self {
        let EntitiesPos {
            obstacles,
            player,
            enemies,
        } = Self::compute_all_entities_position();

        Self {
            equation: String::new(),

            graph_resolution: 100,
            graph_cached_points: None,
            graph_animation_frame: 0,

            player,
            enemies,
            obstacles,

            messages: vec![Message::new(
                "Your are the green thingy, your goal is to aim at the red thingies without touching the purpule thingies".to_string(),
                Duration::from_secs(10),
                UITypes::Info,
            )],
        }
    }
}

// sqrt(x^2 + y^2)

impl GraphWar {
    fn new_game(&mut self) {
        let EntitiesPos {
            obstacles,
            player,
            enemies,
        } = Self::compute_all_entities_position();
        self.obstacles = obstacles;
        self.player = player;
        self.enemies = enemies;
    }

    fn compute_all_entities_position() -> EntitiesPos {
        const ENTITY_AMPLITUDE: f64 = 1.0;

        #[allow(non_snake_case)]
        let distance_bewteen_two_points =
            |A: &PlotPoint, B: &PlotPoint| ((B.x - A.x).powi(2) + (B.y - A.y).powi(2)).sqrt();

        let mut taken_points: Vec<(PlotPoint, f64)> = vec![];
        let does_position_overlap =
            |taken_points: &Vec<(PlotPoint, f64)>, point_to_check: (&PlotPoint, f64)| {
                for taken_point in taken_points {
                    let distance = distance_bewteen_two_points(point_to_check.0, &taken_point.0);
                    if distance <= taken_point.1 + point_to_check.1 {
                        return true;
                    }
                }
                false
            };

        let mut rng = rand::thread_rng();
        let obstacles_nums = rng.gen_range(5..=15);

        let obstacles_sprites = (0..obstacles_nums)
            .map(|_| {
                let mut obstacle_pos = Self::spawn_entity();
                let mut amplitude = rng.gen_range(2..=6) as f64;
                while does_position_overlap(&taken_points, (&obstacle_pos, amplitude)) {
                    obstacle_pos = Self::spawn_entity();
                    amplitude = rng.gen_range(2..=6) as f64;
                }
                taken_points.push((obstacle_pos, amplitude));

                let obstacle_sprite = compute_polygon_points(rng.gen_range(3..=15), amplitude)
                    .points()
                    .to_vec();

                obstacle_sprite
                    .iter()
                    .map(|&PlotPoint { x, y }| [x + obstacle_pos.x, y + obstacle_pos.y])
                    .collect::<PlotPoints>()
                    .points()
                    .to_vec()
            })
            .collect::<Vec<_>>();

        let entity_sprite = compute_polygon_points(100, ENTITY_AMPLITUDE)
            .points()
            .to_vec();

        let mut player_pos = Self::spawn_entity();
        while does_position_overlap(&taken_points, (&player_pos, ENTITY_AMPLITUDE)) {
            player_pos = Self::spawn_entity();
        }
        taken_points.push((player_pos, ENTITY_AMPLITUDE));

        let player_sprite = entity_sprite
            .iter()
            .map(|&PlotPoint { x, y }| [x + player_pos.x, y + player_pos.y])
            .collect::<PlotPoints>()
            .points()
            .to_vec();

        let enemies_nums = rng.gen_range(2..=5);
        let enemies_sprites = (0..enemies_nums)
            .map(|_| {
                let mut ennemy_pos = Self::spawn_entity();
                while does_position_overlap(&taken_points, (&ennemy_pos, ENTITY_AMPLITUDE))
                    || distance_bewteen_two_points(&player_pos, &ennemy_pos) <= 10.0
                {
                    ennemy_pos = Self::spawn_entity();
                }
                taken_points.push((ennemy_pos, ENTITY_AMPLITUDE));

                entity_sprite
                    .iter()
                    .map(|&PlotPoint { x, y }| [x + ennemy_pos.x, y + ennemy_pos.y])
                    .collect::<PlotPoints>()
                    .points()
                    .to_vec()
            })
            .collect::<Vec<_>>();

        EntitiesPos {
            player: (player_sprite, player_pos),
            enemies: enemies_sprites,
            obstacles: obstacles_sprites,
        }
    }

    fn does_it_touch_enemies() {}
    fn does_it_touch_obstacle(points: &Vec<PlotPoint>) -> (bool, usize) {
        (false, 0)
    }

    fn build_graph(&mut self) {
        match MathExpression::new(&self.equation) {
            Ok(math_expr) => {
                let graph_points = compute_line_points(
                    &math_expr,
                    &self.player.1,
                    (-25, 25),
                    self.graph_resolution,
                )
                .points()
                .to_vec();
                self.graph_cached_points = Some(graph_points);
                self.graph_animation_frame = 0;
            }
            Err(_) => {
                self.messages.insert(
                    0,
                    Message::new(
                        "unevaluable mathematical expression".to_string(),
                        Duration::from_secs(4),
                        UITypes::Error,
                    ),
                );
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

impl eframe::App for GraphWar {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let is_messages = !self.messages.is_empty();
        if is_messages {
            self.messages.retain(|msg| !msg.is_expired())
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            let window_size = ui.available_size();

            // PLOT
            ui.vertical_centered_justified(|ui| {
                let max_plot_size = window_size.x.min(window_size.y)
                    - 80.0
                    - if is_messages {
                        25.0 * self.messages.len() as f32
                    } else {
                        0.0
                    };
                let plot = get_app_plot().height(max_plot_size);
                // .width(min_square_size);

                plot.show(ui, |plot_ui| {
                    if let Some(points) = &self.graph_cached_points {
                        plot_ui.render_graph(points, self.graph_animation_frame);
                        if self.graph_animation_frame < points.len() - 1 {
                            self.graph_animation_frame += 100;
                            ctx.request_repaint();
                        }
                    }
                    plot_ui.render_player(&self.player.0);
                    plot_ui.render_ennemies(&self.enemies);
                    plot_ui.render_obstacles(&self.obstacles);
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
