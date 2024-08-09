use std::collections::hash_map::Values;
use std::collections::HashMap;

use crate::rendering_engine::engine::compositor::{composite, Image};
use crate::rendering_engine::engine::projector::project;
use crate::rendering_engine::engine::rasterizer::{rasterize, ZBuffer};
use crate::rendering_engine::engine::z_buffered_triangle::{ZBufferedTriangle, ZBufferedVertex};
use crate::rendering_engine::scene::camera::Camera;
use crate::rendering_engine::scene::camera::display::Display;
use crate::rendering_engine::scene::MeshID;
use crate::rendering_engine::scene::model::color::Color;
use crate::rendering_engine::scene::model_2d::point::Point as Point2D;
use crate::rendering_engine::scene::model_2d::triangle::Triangle as Triangle2D;
use crate::rendering_engine::scene::model_3d::mesh::Mesh;
use crate::rendering_engine::scene::model_3d::plane::Plane;
use crate::rendering_engine::scene::model_3d::plane_direction::PlaneDirection;
use crate::rendering_engine::scene::model_3d::point::Point as Point3D;
use crate::rendering_engine::scene::model_3d::triangle::Triangle as Triangle3D;

pub fn render(camera: &Camera, meshes: Values<MeshID, Mesh>, background_color: Color) -> Image {
    let camera_planes: HashMap<PlaneDirection, Plane> = camera.create_planes();
    let mut z_buffers: Vec<ZBuffer> = Vec::new();

    for triangles in meshes.map(|mesh: &Mesh| mesh.triangulate()) {
        let buffers: Vec<ZBuffer> = triangles
            .iter()
            .map(|triangle3d: &Triangle3D| {
                let triangle2d: Triangle2D = project(&camera_planes, camera.focal_length(), &triangle3d);
                let z_buffered_triangle: ZBufferedTriangle = z_buffer_triangle(&triangle2d, triangle3d, camera.display(), triangle3d.color());
    
                rasterize(&z_buffered_triangle)
            }).collect();

        z_buffers.extend(buffers);
    }

    composite(&z_buffers, camera.display(), background_color)
}

fn z_buffer_triangle(triangle2d: &Triangle2D,
                     triangle3d: &Triangle3D,
                     display: &Display,
                     color: Color) -> ZBufferedTriangle {
    let offset_width: usize = display.width / 2;
    let offset_height: usize = display.height / 2;

    let z_buffered_vertices: Vec<ZBufferedVertex> = triangle2d.vertices
        .iter()
        .zip(triangle3d.vertices().iter())
        .map(|(point2d, point3d): (&Point2D, &Point3D)| {
            ZBufferedVertex { /* convert from cartesian system to bitmap system */
                x: point2d.x + offset_width as f32,
                y: -point2d.y + offset_height as f32,
                distance: point3d.z
            }
        })
        .collect();

    ZBufferedTriangle {
        vertices: z_buffered_vertices.try_into().unwrap(),
        color
    }
}