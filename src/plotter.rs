use egui::{
    plot::{Line, PlotPoints, PlotUi},
    Vec2,
};

use crate::eval::MathExpression;

pub fn get_graph_plot_points(
    math_expr: &MathExpression,
    interval: (isize, isize),
    resolution: usize,
) -> PlotPoints {
    ((interval.0 * resolution as isize)..=(interval.1 * resolution as isize))
        .filter_map(|i| {
            let x = i as f64 * 1.0 / resolution as f64;
            match math_expr.compute(x) {
                Ok(y) => Some([x, y]),
                Err(_) => None,
            }
        })
        .collect()
}

pub trait Plotter {
    fn render_graph(&mut self, math_expr: &MathExpression, graph_res: usize);
    fn render_obstacles(&mut self, obstacles_number: usize);
    fn render_player(&mut self, position: Vec2);
    fn render_ennemies(&mut self, positions: Vec<Vec2>);
}

impl Plotter for PlotUi {
    fn render_graph(&mut self, math_expr: &MathExpression, graph_res: usize) {
        let line = Line::new(get_graph_plot_points(math_expr, (-25, 25), graph_res));
        self.line(line);
    }
    fn render_obstacles(&mut self, obstacles_number: usize) {}
    fn render_player(&mut self, position: Vec2) {
        let Vec2 { x, y } = position;
    }
    fn render_ennemies(&mut self, positions: Vec<Vec2>) {}
}

// This is a circle
// plot_ui.polygon(
//     Polygon::new(PlotPoints::new(
//         (0..1000)
//             .map(|k| k as f64)
//             .map(|k| 2.0 * k * PI / 1000.0) // racine nième de l'unité
//             .map(|x| [x.cos(), x.sin()])
//             .collect(),
//     ))
//     .color(Color32::from_rgb(255, 0, 0)),
// );

// this is y=x, on [-2,2]
// plot_ui.line(Line::new(PlotPoints::new(
//     ((-2 * PLOT_RESOLUTION)..=(2 * PLOT_RESOLUTION))
//         .map(|x| x as f64 / PLOT_RESOLUTION as f64)
//         .map(|x| [x, x])
//         .collect(),
// )));
