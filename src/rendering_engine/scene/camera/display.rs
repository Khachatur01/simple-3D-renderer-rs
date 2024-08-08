use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Display {
    pub width: usize,
    pub height: usize,
}

impl Display {
    pub fn new(width: usize, height: usize) -> Display {
        Display {
            width, height
        }
    }
}
