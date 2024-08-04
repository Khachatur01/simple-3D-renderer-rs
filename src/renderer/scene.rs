use std::ascii::escape_default;
use std::collections::HashMap;
use uuid::Uuid;
use object_3d::mesh::Mesh;
use crate::renderer::scene::camera::Camera;
use crate::renderer::scene::object_2d::point::Point as Point2D;
use crate::renderer::scene::object_3d::triangle::{Triangle as Triangle3D};
use crate::renderer::scene::object_2d::triangle::{Triangle as Triangle2D};
use crate::renderer::scene::object_3d::plane::Plane;
use crate::renderer::scene::object_3d::plane_direction::PlaneDirection;
use crate::renderer::scene::object_3d::point::{Point as Point3D, Point};

pub mod camera;
pub mod object_2d;
pub mod object_3d;

pub type CameraID = Uuid;
pub type ObjectID = Uuid;

pub struct Scene {
    cameras: HashMap<CameraID, Camera>,
    objects: HashMap<ObjectID, Mesh>
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

    pub fn add_mesh(&mut self, mesh: Mesh) -> ObjectID {
        let object_id: ObjectID = Uuid::new_v4();

        self.objects.insert(object_id, mesh);

        object_id
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
            .map(|triangle3d: &Triangle3D| self.project_triangle(&camera_planes, camera.focal_length, &triangle3d))
            .collect();

        Ok(triangles2d)
    }

    #[inline]
    fn project_triangle(&self, camera_planes: &HashMap<PlaneDirection, Plane>, focal_length: f32, triangle: &Triangle3D) -> Triangle2D {
        let vertices: Vec<Point2D> = triangle.vertices
            .iter()
            .map(|vertex: &Point3D| {
                let x_distance: f32 = vertex.distance_from_plane(&camera_planes.get(&PlaneDirection::YZ).unwrap());
                let y_distance: f32 = vertex.distance_from_plane(&camera_planes.get(&PlaneDirection::XZ).unwrap());
                let z_distance: f32 = vertex.distance_from_plane(&camera_planes.get(&PlaneDirection::XY).unwrap());

                if x_distance == 0.0 {
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
            vertices[0],
            vertices[1],
            vertices[2],
        ];

        Triangle2D { vertices }
    }
}
