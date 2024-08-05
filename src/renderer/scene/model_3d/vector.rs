use std::f32::consts::PI;
use crate::renderer::scene::model_3d::axis::Axis;

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

        // let Vector { x, y, z } = &self;

        match axis {
            Axis::X => {
                /*
                * |1     0           0| |x|   |        x        |   |x'|
                * |0   cos θ    −sin θ| |y| = |y cos θ − z sin θ| = |y'|
                * |0   sin θ     cos θ| |z|   |y sin θ + z cos θ|   |z'|
                */

                /* x stays the same */
                self.y = self.y * cos_alpha - self.z * sin_alpha;
                self.z = self.y * sin_alpha + self.z * cos_alpha;
            }
            Axis::Y => {
                /*
                * | cos θ    0   sin θ| |x|   | x cos θ + z sin θ|   |x'|
                * |   0      1       0| |y| = |         y        | = |y'|
                * |−sin θ    0   cos θ| |z|   |−x sin θ + z cos θ|   |z'|
                * */

                self.x = self.x * cos_alpha + self.z * sin_alpha;
                /* y stays the same */
                self.z = -self.x * sin_alpha + self.z * cos_alpha;
            }
            Axis::Z => {
                /*
                * |cos θ   −sin θ   0| |x|   |x cos θ − y sin θ|   |x'|
                * |sin θ    cos θ   0| |y| = |x sin θ + y cos θ| = |y'|
                * |  0       0      1| |z|   |        z        |   |z'|
                * */

                self.x = self.x * cos_alpha - self.y * sin_alpha;
                self.y = self.x * sin_alpha + self.y * cos_alpha;
                /* z stays the same */
            }
        }
    }
}
