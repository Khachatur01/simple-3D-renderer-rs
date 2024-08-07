use std::collections::HashMap;

use crate::rendering_engine::scene::model_2d::point::Point as Point2D;
use crate::rendering_engine::scene::model_2d::triangle::Triangle as Triangle2D;
use crate::rendering_engine::scene::model_3d::plane::Plane;
use crate::rendering_engine::scene::model_3d::plane_direction::PlaneDirection;
use crate::rendering_engine::scene::model_3d::point::Point as Point3D;
use crate::rendering_engine::scene::model_3d::triangle::Triangle as Triangle3D;

pub fn project(camera_planes: &HashMap<PlaneDirection, Plane>, focal_length: f32, triangle: &Triangle3D) -> Triangle2D {
    let vertices: Vec<crate::rendering_engine::scene::model_2d::point::Point> = triangle.vertices()
        .iter()
        .map(|vertex: &Point3D| {
            let x_distance: f32 = vertex.distance_from_plane(&camera_planes.get(&PlaneDirection::YZ).unwrap());
            let y_distance: f32 = vertex.distance_from_plane(&camera_planes.get(&PlaneDirection::XZ).unwrap());
            let z_distance: f32 = vertex.distance_from_plane(&camera_planes.get(&PlaneDirection::XY).unwrap());

            if z_distance == 0.0 {
                return Point2D {
                    x: vertex.x,
                    y: vertex.y,
                };
            }

            return Point2D {
                x: focal_length * x_distance / z_distance,
                y: focal_length * y_distance / z_distance
            }
        })
        .collect::<Vec<Point2D>>();

    let vertices: [Point2D; 3] = [
        Point2D {
            x: vertices[0].x,
            y: vertices[0].y,
        },
        Point2D {
            x: vertices[1].x,
            y: vertices[1].y,
        },
        Point2D {
            x: vertices[2].x,
            y: vertices[2].y,
        }
    ];

    Triangle2D { vertices }
}