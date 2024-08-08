use serde::{Deserialize, Serialize};

use crate::rendering_engine::scene::model::color::Color;

#[derive(Serialize, Deserialize)]
pub struct Face {
    pub vertices: [usize; 3],
    pub color: Color
}

impl Face {
    pub fn new(vertices: [usize; 3], color: Color) -> Self {
        Self {
            vertices, color
        }
    }
}