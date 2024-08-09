use serde::{Deserialize, Serialize};

use crate::rendering_engine::scene::model_3d::face::Face;
use crate::rendering_engine::scene::model_3d::point::Point;
use crate::rendering_engine::scene::model_3d::triangle::Triangle;
use crate::rendering_engine::scene::model_3d::vector::Vector;

#[derive(Serialize, Deserialize)]
pub struct Mesh {
    pub points: Vec<Point>,
    pub faces: Vec<Face>,
}

impl Mesh {
    pub fn move_point(&mut self, point_index: usize, delta: Vector) {
        if let Some(point) = self.points.get_mut(point_index) {
            point.x += delta.x;
            point.y += delta.y;
            point.z += delta.z;
        }
    }

    pub fn triangulate(&self) -> Vec<Triangle> {
        self.faces.iter()
            .map(|face: &Face| {
                Triangle::new([
                    self.points[face.vertices[0]].clone(),
                    self.points[face.vertices[1]].clone(),
                    self.points[face.vertices[2]].clone()
                ], face.color)
            })
            .collect()
    }
}
