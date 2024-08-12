use serde::{Deserialize, Serialize};

#[derive(Copy, Clone)]
#[derive(Serialize, Deserialize)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Pixel {
    pub fn new(r: u8, g: u8, b: u8) -> Pixel {
        Pixel {
            r, g, b
        }
    }
}

impl Default for Pixel {
    fn default() -> Self {
        Self::new(0, 0, 0)
    }
}