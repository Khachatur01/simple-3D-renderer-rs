use serde::{Deserialize, Serialize};

use crate::renderer::scene::model_3d::point::Point;

pub type TriangleVertices = [Point; 3];


#[derive(Serialize, Deserialize)]
pub struct Triangle {
    vertices: TriangleVertices
}

impl Triangle {
    pub fn new(vertices: TriangleVertices) -> Triangle {
        Triangle {
            vertices
        }
    }

    pub fn vertices(&self) -> TriangleVertices {
        self.vertices
    }
}
