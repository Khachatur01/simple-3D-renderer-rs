use serde::{Deserialize, Serialize};

use crate::rendering_engine::scene::model::color::Color;
use crate::rendering_engine::scene::model_3d::point::Point;

pub type TriangleVertices = [Point; 3];


#[derive(Serialize, Deserialize)]
pub struct Triangle {
    vertices: TriangleVertices,
    color: Color,
}

impl Triangle {
    pub fn new(vertices: TriangleVertices, color: Color) -> Self {
        Self {
            vertices, color
        }
    }

    pub fn vertices(&self) -> TriangleVertices {
        self.vertices
    }

    pub fn color(&self) -> Color {
        self.color
    }
}
