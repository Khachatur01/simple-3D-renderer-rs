use serde::{Deserialize, Serialize};

#[derive(Copy, Clone)]
#[derive(Serialize, Deserialize)]
pub struct Coefficients {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
}