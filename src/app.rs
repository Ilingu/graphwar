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
        let polygon_bounds = |amplitude: f64, pos: &PlotPoint| {
            (
                (amplitude + pos.x, pos.y), // cos(0) -> rightmost point
                (pos.x - amplitude, pos.y), // cos(PI) -> leftmost point
                (pos.x, amplitude + pos.y), // sin(PI/2) -> topmost point
                (pos.x, pos.y - amplitude), // sin(-PI/2) -> bottom most point
            )
        };

        type Bounds = ((f64, f64), (f64, f64), (f64, f64), (f64, f64));
        let mut taken_positions: Vec<Bounds> = vec![];
        let does_position_overlap = |taken_pos: &Vec<Bounds>, (rmi, lmi, tmi, bmi): Bounds| {
            for (rmt, lmt, tmt, bmt) in taken_pos {
                let is_point_inside =
                    |p: &(f64, f64)| rmt.0 > p.0 && lmt.0 < p.0 && tmt.1 > p.1 && bmt.1 < p.1;

                if is_point_inside(&rmi)
                    || is_point_inside(&lmi)
                    || is_point_inside(&tmi)
                    || is_point_inside(&bmi)
                {
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
                let amplitude = rng.gen_range(2..=6) as f64;
                while does_position_overlap(
                    &taken_positions,
                    polygon_bounds(amplitude, &obstacle_pos),
                ) {
                    obstacle_pos = Self::spawn_entity();
                }
                taken_positions.push(polygon_bounds(amplitude, &obstacle_pos));

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
        while does_position_overlap(
            &taken_positions,
            polygon_bounds(ENTITY_AMPLITUDE, &player_pos),
        ) {
            player_pos = Self::spawn_entity();
        }
        taken_positions.push(polygon_bounds(ENTITY_AMPLITUDE, &player_pos));

        let player_sprite = entity_sprite
            .iter()
            .map(|&PlotPoint { x, y }| [x + player_pos.x, y + player_pos.y])
            .collect::<PlotPoints>()
            .points()
            .to_vec();

        let distance_to_player = |pos: &PlotPoint| {
            ((pos.x - player_pos.x).powi(2) + (pos.y - player_pos.y).powi(2)).sqrt()
        };

        let enemies_nums = rng.gen_range(2..=5);
        let enemies_sprites = (0..enemies_nums)
            .map(|_| {
                let mut ennemy_pos = Self::spawn_entity();
                while does_position_overlap(
                    &taken_positions,
                    polygon_bounds(ENTITY_AMPLITUDE, &ennemy_pos),
                ) && distance_to_player(&ennemy_pos) <= 10.0
                {
                    ennemy_pos = Self::spawn_entity();
                }
                taken_positions.push(polygon_bounds(ENTITY_AMPLITUDE, &ennemy_pos));

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

    fn build_graph(&mut self) {
        match MathExpression::new(&self.equation) {
            Ok(math_expr) => {
                self.graph_cached_points = Some(
                    compute_line_points(
                        &math_expr,
                        &self.player.1,
                        (-25, 25),
                        self.graph_resolution,
                    )
                    .points()
                    .to_vec(),
                );
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
                        plot_ui.render_graph(points);
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
