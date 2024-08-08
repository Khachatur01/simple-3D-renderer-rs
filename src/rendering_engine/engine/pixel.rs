use serde::{Deserialize, Serialize};

use crate::rendering_engine::scene::model::color::Color;

#[derive(Copy, Clone)]
#[derive(Serialize, Deserialize)]
pub struct Pixel {
    pub color: Color
}

impl Pixel {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Pixel {
        Pixel {
            color: Color { r, g, b, a }
        }
    }
}

impl Default for Pixel {
    fn default() -> Self {
        Self::new(0, 0, 0, 0)
    }
}