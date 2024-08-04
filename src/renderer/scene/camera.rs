pub mod display;

use std::collections::HashMap;
use crate::renderer::scene::camera::display::Display;
use crate::renderer::scene::object_3d::axis::Axis;
use crate::renderer::scene::object_3d::plane::Plane;
use crate::renderer::scene::object_3d::plane_direction::PlaneDirection;
use crate::renderer::scene::object_3d::point::Point;
use crate::renderer::scene::object_3d::vector::Vector;

pub struct Camera {
    pub focal_length: f32,
    pub center: Point,
    pub pitch_angle: f32, /* x axis */
    pub yaw_angle: f32, /* y axis */
    pub roll_angle: f32, /* z axis */
    pub display: Display
}

impl Camera {
    pub fn reposition(&mut self, delta: Vector) {

    }

    pub fn rotate(&mut self, delta: Vector) {

    }

    pub fn move_focal_length(&mut self, delta: Vector) {

    }

    pub fn create_planes(&self) -> HashMap<PlaneDirection, Plane> {
        let mut yz_plane_normal: Vector = Vector { x: 1.0, y: 0.0, z: 0.0 };
        let mut xz_plane_normal: Vector = Vector { x: 0.0, y: 1.0, z: 0.0 };
        let mut xy_plane_normal: Vector = Vector { x: 0.0, y: 0.0, z: 1.0 };

        yz_plane_normal.rotate(Axis::Y, self.yaw_angle);
        yz_plane_normal.rotate(Axis::Z, self.roll_angle);
        yz_plane_normal.rotate(Axis::X, self.pitch_angle);

        xz_plane_normal.rotate(Axis::X, self.pitch_angle);
        xz_plane_normal.rotate(Axis::Z, self.roll_angle);
        xz_plane_normal.rotate(Axis::Y, self.yaw_angle);

        xy_plane_normal.rotate(Axis::X, self.pitch_angle);
        xy_plane_normal.rotate(Axis::Y, self.yaw_angle);
        xy_plane_normal.rotate(Axis::Z, self.roll_angle);

        let mut planes = HashMap::with_capacity(3);

        planes.insert(PlaneDirection::YZ, Plane::new(yz_plane_normal, self.center));
        planes.insert(PlaneDirection::XZ, Plane::new(xz_plane_normal, self.center));
        planes.insert(PlaneDirection::XY, Plane::new(xy_plane_normal, self.center));

        return planes;
    }
}