use serde::{Deserialize, Serialize};

use crate::renderer::scene::model_3d::plane::coefficients::Coefficients;
use crate::renderer::scene::model_3d::point::Point;
use crate::renderer::scene::model_3d::vector::Vector;

pub mod coefficients;

#[derive(Serialize, Deserialize)]
pub struct Plane {
    /** coefficients of plane equation */
    pub coefficients: Coefficients
}

impl Plane {
    pub fn new(normal: Vector, point: Point) -> Plane {
        let Vector { x: a, y: b, z: c } = normal;

        let d: f32 = a*point.x + b*point.y + c*point.z;

        Plane {
            coefficients: Coefficients { a, b, c, d, }
        }
    }
}