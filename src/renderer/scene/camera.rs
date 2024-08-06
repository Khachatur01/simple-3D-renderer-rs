use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::renderer::scene::camera::display::Display;
use crate::renderer::scene::model_3d::axis::Axis;
use crate::renderer::scene::model_3d::plane::Plane;
use crate::renderer::scene::model_3d::plane_direction::PlaneDirection;
use crate::renderer::scene::model_3d::point::Point;
use crate::renderer::scene::model_3d::vector::Vector;

pub mod display;

#[derive(Serialize, Deserialize)]
pub struct Camera {
    focal_length: f32,
    center: Point,
    pitch_angle: f32, /* x axis */
    yaw_angle: f32, /* y axis */
    roll_angle: f32, /* z axis */
    display: Display
}

impl Camera {
    pub fn new(focal_length: f32,
               center: Point,
               pitch_angle: f32,
               yaw_angle: f32,
               roll_angle: f32,
               display: Display) -> Camera {

        Camera {
            focal_length, center, pitch_angle, yaw_angle, roll_angle, display
        }
    }

    pub fn reposition(&mut self, mut delta: Vector) {
        delta.rotate(Axis::X, self.pitch_angle);
        delta.rotate(Axis::Y, self.yaw_angle);
        delta.rotate(Axis::Z, self.roll_angle);

        self.center.x += delta.x;
        self.center.y += delta.y;
        self.center.z += delta.z;
    }

    pub fn rotate(&mut self, delta: &Vector) {
        self.pitch_angle += delta.x;
        self.yaw_angle += delta.y;
        self.roll_angle += delta.z;

        self.pitch_angle %= 360.0;
        self.yaw_angle %= 360.0;
        self.roll_angle %= 360.0;
    }

    pub fn move_focal_length(&mut self, delta: f32) {
        self.focal_length += delta;
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

    pub fn focal_length(&self) -> f32 {
        self.focal_length
    }

    pub fn center(&self) -> &Point {
        &self.center
    }

    pub fn pitch_angle(&self) -> f32 {
        self.pitch_angle
    }

    pub fn yaw_angle(&self) -> f32 {
        self.yaw_angle
    }

    pub fn roll_angle(&self) -> f32 {
        self.roll_angle
    }

    pub fn display(&self) -> &Display {
        &self.display
    }
}