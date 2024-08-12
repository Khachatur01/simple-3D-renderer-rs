use serde::{Deserialize, Serialize};

use crate::rendering_engine::scene::model::color::Color;

#[derive(Copy, Clone)]
#[derive(Serialize, Deserialize)]
pub struct DepthPixel {
    pub color: Color,
    pub depth: f32, /* value should be from 0 to 1 */
}

impl DepthPixel {
    pub fn new(r: u8, g: u8, b: u8, a: f32) -> Self {
        DepthPixel {
            color: Color::new(r, g, b, a),
            depth: 0.0
        }
    }
}

impl Default for DepthPixel {
    fn default() -> Self {
        DepthPixel {
            color: Color::new(0, 0, 0, 0.0),
            depth: 0.0
        }
    }
}