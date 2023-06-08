use std::f64::consts::PI;

use egui::{
    plot::{Line, Plot, PlotPoint, PlotPoints, PlotUi, Polygon},
    Color32,
};

use crate::eval::MathExpression;

pub fn compute_line_points(
    math_expr: &MathExpression,
    from_point: &PlotPoint,
    interval: (isize, isize),
    resolution: usize,
) -> PlotPoints {
    (((interval.0 + from_point.x.abs() as isize) * resolution as isize)
        ..=((interval.1 + from_point.x.abs() as isize) * resolution as isize))
        .filter_map(|i| {
            let x = i as f64 * 1.0 / resolution as f64;
            match math_expr.compute(x) {
                Ok(y) => Some([x + from_point.x, y + from_point.y]),
                Err(_) => None,
            }
        })
        .collect()
}

pub fn compute_polygon_points(n_gon: usize, amplitude: f64) -> PlotPoints {
    PlotPoints::new(
        (0..n_gon)
            .map(|k| 2.0 * k as f64 * PI / n_gon as f64) // racine nième de l'unité
            .map(|x| [amplitude * x.cos(), amplitude * x.sin()])
            .collect(),
    )
}

pub fn get_app_plot() -> Plot {
    Plot::new("game_graph")
        // block all plot camera movements
        .allow_zoom(false)
        .allow_drag(false)
        .allow_boxed_zoom(false)
        .allow_scroll(false)
        // block plot bounds to [-25, 25] in the x and y axes
        .include_x(-25.0)
        .include_x(25.0)
        .include_y(-25.0)
        .include_y(25.0)
}

pub trait Plotter {
    fn render_graph(&mut self, points: &[PlotPoint]);
    fn render_obstacles(&mut self, sprites: &[Vec<PlotPoint>]);
    fn render_player(&mut self, sprite: &[PlotPoint]);
    fn render_ennemies(&mut self, sprites: &[Vec<PlotPoint>]);
}

impl Plotter for PlotUi {
    fn render_graph(&mut self, points: &[PlotPoint]) {
        let points: PlotPoints = points.iter().map(|&PlotPoint { x, y }| [x, y]).collect();
        self.line(Line::new(points).width(2.0));
    }
    fn render_obstacles(&mut self, sprites: &[Vec<PlotPoint>]) {
        for sprite in sprites {
            let sprite_series: PlotPoints =
                sprite.iter().map(|&PlotPoint { x, y }| [x, y]).collect();
            self.polygon(Polygon::new(sprite_series).color(Color32::from_rgb(152, 115, 172)));
        }
    }
    fn render_player(&mut self, sprite: &[PlotPoint]) {
        let sprite_series: PlotPoints = sprite.iter().map(|&PlotPoint { x, y }| [x, y]).collect();
        self.polygon(Polygon::new(sprite_series).color(Color32::LIGHT_GREEN));
    }
    fn render_ennemies(&mut self, sprites: &[Vec<PlotPoint>]) {
        for sprite in sprites {
            let sprite_series: PlotPoints =
                sprite.iter().map(|&PlotPoint { x, y }| [x, y]).collect();
            self.polygon(Polygon::new(sprite_series).color(Color32::LIGHT_RED));
        }
    }
}
