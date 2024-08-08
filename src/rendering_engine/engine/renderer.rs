use std::collections::hash_map::Values;
use std::collections::HashMap;

use crate::log;
use crate::rendering_engine::engine::compositor::{composite, Image};
use crate::rendering_engine::engine::projector::project;
use crate::rendering_engine::engine::rasterizer::{rasterize, ZBuffer};
use crate::rendering_engine::scene::camera::Camera;
use crate::rendering_engine::scene::MeshID;
use crate::rendering_engine::scene::model_2d::triangle::Triangle as Triangle2D;
use crate::rendering_engine::scene::model_3d::mesh::Mesh;
use crate::rendering_engine::scene::model_3d::plane::Plane;
use crate::rendering_engine::scene::model_3d::plane_direction::PlaneDirection;
use crate::rendering_engine::scene::model_3d::triangle::Triangle as Triangle3D;

pub fn render(camera: &Camera, meshes: Values<MeshID, Mesh>) -> Image {
    let camera_planes: HashMap<PlaneDirection, Plane> = camera.create_planes();

    let z_buffers: Vec<ZBuffer> = meshes
        .map(|mesh: &Mesh| mesh.triangulate())
        .last()
        .unwrap_or(vec![])
        .iter()
        .map(|triangle3d: &Triangle3D| {
            let triangle2d: Triangle2D = project(&camera_planes, camera.focal_length(), &triangle3d);
            log(serde_json::to_string(&triangle2d).unwrap().as_str());

            rasterize(&triangle2d, triangle3d)
        })
        .collect();

    composite(&z_buffers, camera.display())
}