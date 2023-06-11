use std::time::Duration;

use egui::plot::{PlotPoint, PlotPoints};

use evalexpr::EvalexprError;
use rand::Rng;

use crate::{
    eval::MathExpression,
    plotter::{compute_line_points, compute_polygon_points, get_app_plot, Plotter},
    ui::{rich_text, Message, UITypes},
};

struct EntitiesPos {
    player: (Vec<PlotPoint>, PlotPoint),       // (sprite, position)
    enemies: Vec<(Vec<PlotPoint>, PlotPoint)>, // Vec<(sprite, position)>
    obstacles: Vec<(Vec<PlotPoint>, PlotPoint, f64)>, // Vec<(sprite, position, amplitude)>
}

#[derive(PartialEq, Debug)]
enum CollisionType {
    Obstacle,
    Ennemy,
}

#[derive(PartialEq, Debug)]
struct Collision {
    entity_point: PlotPoint,
    frame_id: usize,
    collision_type: CollisionType,
    entity_id: usize,
}

#[allow(non_snake_case)]
fn distance_bewteen_two_points(A: &PlotPoint, B: &PlotPoint) -> f64 {
    ((B.x - A.x).powi(2) + (B.y - A.y).powi(2)).sqrt()
}

const ENTITY_AMPLITUDE: f64 = 1.0;

pub struct GraphWar {
    equation: String,

    graph_cached_points: Option<Vec<PlotPoint>>,
    graph_animation_frame: usize,
    graph_animation_speed: usize,
    graph_resolution: usize,
    enemies_killed: Vec<(PlotPoint, usize)>, // (enemy_pos, frame_id)

    player: (Vec<PlotPoint>, PlotPoint), // (sprite, position)
    enemies: Vec<(Vec<PlotPoint>, PlotPoint)>, // Vec<(sprite, position)>
    obstacles: Vec<(Vec<PlotPoint>, PlotPoint, f64)>, // Vec<(sprite, position, amplitude)>

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
            graph_animation_speed: 85,
            enemies_killed: vec![],

            player,
            enemies,
            obstacles,

            messages: vec![Message::new(
                "Your are the green thingy, your goal is to aim at the red thingies without touching the purpule thingies".to_string(),
                Duration::from_secs(6),
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
        self.enemies_killed = vec![];
        self.graph_cached_points = None;
        self.graph_animation_frame = 0;
    }

    fn compute_all_entities_position() -> EntitiesPos {
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

                // rng.gen_range(3..=15)
                let obstacle_sprite = compute_polygon_points(20, amplitude).points().to_vec();

                (
                    obstacle_sprite
                        .iter()
                        .map(|&PlotPoint { x, y }| [x + obstacle_pos.x, y + obstacle_pos.y])
                        .collect::<PlotPoints>()
                        .points()
                        .to_vec(),
                    obstacle_pos,
                    amplitude,
                )
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

                (
                    entity_sprite
                        .iter()
                        .map(|&PlotPoint { x, y }| [x + ennemy_pos.x, y + ennemy_pos.y])
                        .collect::<PlotPoints>()
                        .points()
                        .to_vec(),
                    ennemy_pos,
                )
            })
            .collect::<Vec<_>>();

        EntitiesPos {
            player: (player_sprite, player_pos),
            enemies: enemies_sprites,
            obstacles: obstacles_sprites,
        }
    }

    /// given all the graph points, detect if it touches obstacles or enemies
    fn detect_collision(&self, points: &[PlotPoint]) -> Option<Vec<Collision>> {
        let mut collisions = vec![];
        for (frame_id, point) in points.iter().enumerate() {
            let is_collision = |entity_point: &PlotPoint, amplitude: f64| {
                let distance = distance_bewteen_two_points(point, entity_point);
                distance <= amplitude
            };

            for (ennemy_id, (_, ennemy_pos)) in self.enemies.iter().enumerate() {
                if is_collision(ennemy_pos, ENTITY_AMPLITUDE) {
                    let collision = Collision {
                        entity_point: *ennemy_pos,
                        collision_type: CollisionType::Ennemy,
                        frame_id,
                        entity_id: ennemy_id,
                    };
                    if !collisions.contains(&collision) {
                        collisions.push(collision);
                    }
                }
            }
            for (obstacle_id, (_, obstacle_pos, amplitude)) in self.obstacles.iter().enumerate() {
                if is_collision(obstacle_pos, *amplitude) {
                    let collision = Collision {
                        entity_point: *obstacle_pos,
                        collision_type: CollisionType::Obstacle,
                        frame_id,
                        entity_id: obstacle_id,
                    };
                    if !collisions.contains(&collision) {
                        collisions.push(collision);
                    }
                }
            }
        }

        if collisions.is_empty() {
            None
        } else {
            Some(collisions)
        }
    }

    fn build_graph(&mut self) {
        match MathExpression::new(&self.equation) {
            Ok(math_expr) => {
                let mut graph_points = compute_line_points(
                    &math_expr,
                    &self.player.1,
                    (-25, 25),
                    self.graph_resolution,
                )
                .points()
                .to_vec();

                if let Some(collisions) = self.detect_collision(&graph_points) {
                    for collision in collisions {
                        match collision.collision_type {
                            CollisionType::Obstacle => {
                                // when a obstacle is encounter it's the graph end. Thus we stop the graph points at this place
                                graph_points = graph_points[..=collision.frame_id].to_vec();
                                break;
                            }
                            CollisionType::Ennemy => self
                                .enemies_killed
                                .push((collision.entity_point, collision.frame_id)),
                        }
                    }
                }

                self.graph_cached_points = Some(graph_points);
                self.graph_animation_frame = 0;
            }
            Err(why) => {
                let reason = match why {
                    EvalexprError::CustomMessage(reason) => reason,
                    _ => "unevaluable mathematical expression".to_string(),
                };
                self.messages.insert(
                    0,
                    Message::new(reason, Duration::from_secs(4), UITypes::Error),
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
        if self.enemies.is_empty() {
            self.new_game();
        }

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
                        // animation manager: while current frame is not equal to the last frame, continue animation
                        if self.graph_animation_frame < points.len() - 1 {
                            // delete enemies from app and UI and the animation touch them
                            {
                                // get all the enemies touched before the nth frame...
                                let enemies_touched = self
                                    .enemies_killed
                                    .iter()
                                    .filter(|(_, frame_id)| frame_id <= &self.graph_animation_frame)
                                    .map(|(entity_pos, _)| *entity_pos)
                                    .collect::<Vec<_>>();
                                // ...and delete them
                                self.enemies
                                    .retain(|(_, pos)| !enemies_touched.contains(pos));
                            }

                            self.graph_animation_frame += self.graph_animation_speed;
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
