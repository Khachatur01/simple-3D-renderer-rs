use std::collections::hash_map::Values;
use std::collections::HashMap;

use uuid::Uuid;

use camera::Camera;
use model_3d::mesh::Mesh;
use model_3d::point::Point as Point3D;

use crate::rendering_engine::scene::model::color::Color;
use crate::rendering_engine::scene::model_3d::face::Face;

pub mod camera;
pub mod model_2d;
pub mod model_3d;
pub mod model;

pub type CameraID = Uuid;
pub type MeshID = Uuid;

pub struct Scene {
    cameras: HashMap<CameraID, Camera>,
    meshes: HashMap<MeshID, Mesh>
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            cameras: HashMap::new(),
            meshes: HashMap::new()
        }
    }

    pub fn get_camera(&self, camera_id: CameraID) -> Option<&Camera> {
        self.cameras.get(&camera_id)
    }

    pub fn get_camera_mut(&mut self, camera_id: CameraID) -> Option<&mut Camera> {
        self.cameras.get_mut(&camera_id)
    }

    pub fn get_mesh(&self, mesh_id: MeshID) -> Option<&Mesh> {
        self.meshes.get(&mesh_id)
    }

    pub fn get_all_meshes(&self) -> Values<'_, MeshID, Mesh> {
        self.meshes.values()
    }

    pub fn add_camera(&mut self, camera: Camera) -> CameraID {
        let camera_id: CameraID = Uuid::new_v4();

        self.cameras.insert(camera_id, camera);

        camera_id
    }

    pub fn add_mesh(&mut self, points: Vec<Point3D>, faces: Vec<Face>) -> MeshID {
        let object_id: MeshID = Uuid::new_v4();

        self.meshes.insert(object_id, Mesh { points, faces });

        object_id
    }

    pub fn add_cube(&mut self, position: Point3D, width: f32, height: f32, length: f32, color: Color) -> MeshID {
        let width: f32 = width / 2.0;
        let height: f32 = height / 2.0;
        let length: f32 = length / 2.0;

        let Point3D { x, y, z } = position;

        let points: Vec<Point3D> = vec![
            /* front face */
            Point3D { x: x - width, y: y - height, z: z - length }, /* 0 bottom left */
            Point3D { x: x - width, y: y + height, z: z - length }, /* 1 top left */
            Point3D { x: x + width, y: y + height, z: z - length }, /* 2 top right */
            Point3D { x: x + width, y: y - height, z: z - length }, /* 3 bottom right */

            /* back face */
            Point3D { x: x - width, y: y - height, z: z + length }, /* 4 bottom left */
            Point3D { x: x - width, y: y + height, z: z + length }, /* 5 top left */
            Point3D { x: x + width, y: y + height, z: z + length }, /* 6 top right */
            Point3D { x: x + width, y: y - height, z: z + length }, /* 7 bottom right */
        ];

        let faces: Vec<Face> = vec![
            /* front face */
            Face::new([0, 1, 2], color), Face::new([0, 2, 3], color),
            /* back face */
            Face::new([4, 5, 6], color), Face::new([4, 6, 7], color),
            /* left face */
            Face::new([0, 1, 4], color), Face::new([1, 5, 4], color),
            /* top face */
            Face::new([1, 2, 5], color), Face::new([2, 5, 6], color),
            /* left face */
            Face::new([2, 6, 3], color), Face::new([3, 6, 7], color),
            /* bottom face */
            Face::new([0, 4, 7], color), Face::new([0, 7, 3], color),
        ];

        self.add_mesh(points, faces)
    }
}
