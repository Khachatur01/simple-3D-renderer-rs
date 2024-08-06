use serde::{Deserialize, Serialize};

use crate::renderer::scene::model_2d::point::Point;

pub type TriangleVertices = [Point; 3];

#[derive(Serialize, Deserialize)]
pub struct Triangle {
    pub vertices: TriangleVertices
}

impl Triangle {
    pub fn new(vertices: TriangleVertices) -> Triangle {
        Triangle {
            vertices
        }
    }
}
