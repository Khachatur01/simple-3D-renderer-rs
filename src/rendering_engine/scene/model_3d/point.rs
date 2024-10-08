use serde::{Deserialize, Serialize};

use crate::rendering_engine::scene::model_3d::plane::coefficients::Coefficients;
use crate::rendering_engine::scene::model_3d::plane::Plane;

#[derive(Debug, Copy, Clone)]
#[derive(Serialize, Deserialize)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point {
    pub fn distance_from_plane(&self, plane: &Plane) -> f32 {
        let Coefficients { a, b, c, d } = plane.coefficients;
        let Self { x, y, z } = self.clone();

        let numerator: f32 = a*x + b*y + c*z + d;
        let sum_of_squares: f32 = a.powi(2) + b.powi(2) + c.powi(2);

        numerator / sum_of_squares.sqrt()
    }
}
