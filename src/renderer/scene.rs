use std::collections::HashMap;

use uuid::Uuid;

use crate::renderer::scene::camera::Camera;
use crate::renderer::scene::model_2d::pixel::Pixel;
use crate::renderer::scene::model_2d::point::Point as Point2D;
use crate::renderer::scene::model_2d::triangle::Triangle as Triangle2D;
use crate::renderer::scene::model_3d::mesh::Mesh;
use crate::renderer::scene::model_3d::plane::Plane;
use crate::renderer::scene::model_3d::plane_direction::PlaneDirection;
use crate::renderer::scene::model_3d::point::Point as Point3D;
use crate::renderer::scene::model_3d::triangle::Triangle as Triangle3D;

pub mod camera;
pub mod model_2d;
pub mod model_3d;

pub type CameraID = Uuid;
pub type MeshID = Uuid;

pub struct Scene {
    cameras: HashMap<CameraID, Camera>,
    objects: HashMap<MeshID, Mesh>
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            cameras: HashMap::new(),
            objects: HashMap::new()
        }
    }

    pub fn add_camera(&mut self, camera: Camera) -> CameraID {
        let camera_id: CameraID = Uuid::new_v4();

        self.cameras.insert(camera_id, camera);

        return camera_id;
    }

    pub fn get_camera(&mut self, camera_id: CameraID) -> Option<&mut Camera> {
        self.cameras.get_mut(&camera_id)
    }

    pub fn get_mesh(&self, mesh_id: MeshID) -> Option<&Mesh> {
        self.objects.get(&mesh_id)
    }

    pub fn add_mesh(&mut self, points: Vec<Point3D>, faces: Vec<[usize; 3]>) -> MeshID {
        let object_id: MeshID = Uuid::new_v4();

        self.objects.insert(object_id, Mesh { points, faces });

        object_id
    }

    pub fn add_cube(&mut self, position: Point3D, width: f32, height: f32, length: f32) -> MeshID {
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

        let faces: Vec<[usize; 3]> = vec![
            /* front face */
            [0, 1, 2], [0, 2, 3],
            /* back face */
            [4, 5, 6], [4, 6, 7],
            /* left face */
            [0, 1, 4], [1, 5, 4],
            /* top face */
            [1, 2, 5], [2, 5, 6],
            /* left face */
            [2, 6, 3], [3, 6, 7],
            /* bottom face */
            [0, 4, 7], [0, 7, 3],
        ];

        self.add_mesh(points, faces)
    }

    pub fn render(&self, camera_id: CameraID) -> Result<Vec<Triangle2D>, &str> {
        let camera: &Camera = self.cameras.get(&camera_id).ok_or("Camera not found")?;

        let camera_planes: HashMap<PlaneDirection, Plane> = camera.create_planes();

        let triangles2d: Vec<Triangle2D> = self.objects
            .iter()
            .map(|entry| entry.1)
            .map(|mesh: &Mesh| mesh.triangulate())
            .last()
            .unwrap_or(vec![])
            .iter()
            .map(|triangle3d: &Triangle3D| Self::project_triangle(&camera_planes, camera.focal_length(), &triangle3d))
            .collect();

        Ok(triangles2d)
    }

    #[inline]
    fn project_triangle(camera_planes: &HashMap<PlaneDirection, Plane>, focal_length: f32, triangle: &Triangle3D) -> Triangle2D {
        let vertices: Vec<Point2D> = triangle.vertices()
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

    fn rasterize(&self, triangles: Vec<Triangle2D>, width: usize, height: usize) -> Vec<Pixel> {
        let size: usize = width * height;
        let buffer: Vec<Pixel> = Vec::with_capacity(size);



        buffer
    }
}
