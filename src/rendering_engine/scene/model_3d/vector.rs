use std::f32::consts::PI;

use serde::{Deserialize, Serialize};

use crate::rendering_engine::scene::model_3d::axis::Axis;

#[derive(Serialize, Deserialize)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector {
    pub fn rotate(&mut self, axis: Axis, angle: f32) {
        let angle_radian: f32 = (PI * angle) / 180.0;

        let sin_alpha: f32 = angle_radian.sin();
        let cos_alpha: f32 = angle_radian.cos();

        match axis {
            Axis::X => {
                /*
                * |  0       0      1| |x|   |        x        |   |x'|
                * |cos θ   −sin θ   0| |y|   |y cos θ − z sin θ|   |y'|
                * |sin θ    cos θ   0| |z| = |y sin θ + z cos θ| = |z'|
                * */
                let y = self.y * cos_alpha - self.z * sin_alpha;
                let z = self.y * sin_alpha + self.z * cos_alpha;

                /* x stays the same */
                self.y = y;
                self.z = z;
            }
            Axis::Y => {
                /*
                * | cos θ    0   sin θ| |x|   | x cos θ + z sin θ|   |x'|
                * |   0      y       0| |y| = |         y        | = |y'|
                * |−sin θ    0   cos θ| |z|   |−x sin θ + z cos θ|   |z'|
                * */
                let x = self.x * cos_alpha + self.z * sin_alpha;
                let z = -self.x * sin_alpha + self.z * cos_alpha;

                self.x = x;
                /* y stays the same */
                self.z = z;
            }
            Axis::Z => {
                /*
                * |0   cos θ    −sin θ| |x| = |x cos θ − y sin θ| = |x'|
                * |0   sin θ     cos θ| |y|   |x sin θ + y cos θ|   |y'|
                * |1     0           0| |z|   |        z        |   |z'|
                * */
                let x = self.x * cos_alpha - self.y * sin_alpha;
                let y = self.x * sin_alpha + self.y * cos_alpha;

                self.x = x;
                self.y = y;
                /* z stays the same */
            }
        }
    }
}
