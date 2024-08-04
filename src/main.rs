use crate::renderer::{Renderer, SceneId};
use crate::renderer::scene::camera::Camera;
use crate::renderer::scene::camera::display::Display;
use crate::renderer::scene::object_3d::point::{Point as Point3D};
use crate::renderer::scene::{CameraID, Scene};
use crate::renderer::scene::object_3d::mesh::Mesh;
use crate::renderer::scene::object_2d::triangle::Triangle as Triangle2D;

mod renderer;

fn main() {
    let mut renderer: Renderer = Renderer::new();
    let scene_id: SceneId = renderer.create_scene();

    let scene: &mut Scene = renderer.get_scene(scene_id).unwrap();

    let width: f32 = 800.0;
    let height: f32 = 800.0;

    let camera_id: CameraID = scene.add_camera(Camera {
        focal_length: width * 2.0,
        center: Point3D { x: 0.0, y: 0.0, z: 0.0 },
        roll_angle: 0.0,
        pitch_angle: 0.0,
        yaw_angle: 0.0,
        display: Display { width, height }
    });

    scene.add_mesh(Mesh::new(vec![
        Point3D { x: -150.0, y: -50.0, z: 250.0 }, /* bottom left */
        Point3D { x: -150.0, y: 50.0, z: 250.0 }, /* top left */
        Point3D { x: -50.0, y: 50.0, z: 250.0 }, /* top right */
    ]));

    let triangles: Vec<Triangle2D> = scene.render(camera_id).unwrap();

    for triangle in triangles {
        for vertex in triangle.vertices {
            print!("({}, {}) ", vertex.x, vertex.y);
        }

        println!();
    }

    println!("Hello, world!");
}
