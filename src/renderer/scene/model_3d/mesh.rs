use serde::{Deserialize, Serialize};
use crate::renderer::scene::model_3d::point::Point;
use crate::renderer::scene::model_3d::triangle::Triangle;
use crate::renderer::scene::model_3d::vector::Vector;


#[derive(Serialize, Deserialize)]
pub struct Mesh {
    pub points: Vec<Point>,
    pub faces: Vec<[usize; 3]>,
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
            .map(|face: &[usize; 3]| {
                Triangle::new([
                    self.points[face[0]],
                    self.points[face[1]],
                    self.points[face[2]]
                ])
            })
            .collect()
    }
}
