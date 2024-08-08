use serde::{Deserialize, Serialize};

use crate::rendering_engine::engine::pixel::Pixel;

#[derive(Copy, Clone)]
#[derive(Serialize, Deserialize)]
pub struct DepthPixel {
    pub pixel: Pixel,
    pub depth: f32, /* value should be from 0 to 1 */
}

impl Default for DepthPixel {
    fn default() -> Self {
        DepthPixel {
            pixel: Pixel::new(0, 0, 0, 0),
            depth: 0.0
        }
    }
}