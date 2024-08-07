use std::collections::HashMap;

use uuid::Uuid;

use scene::Scene;

use crate::rendering_engine::engine::compositor::Image;
use crate::rendering_engine::engine::renderer::render;
use crate::rendering_engine::scene::camera::Camera;
use crate::rendering_engine::scene::CameraID;
use crate::rendering_engine::scene::model_3d::plane::Plane;
use crate::rendering_engine::scene::model_3d::plane_direction::PlaneDirection;

pub mod scene;
mod engine;

pub type SceneId = Uuid;

pub struct RenderingEngine {
    scenes: HashMap<SceneId, Scene>
}

impl RenderingEngine {
    pub fn new() -> RenderingEngine {
        RenderingEngine {
            scenes: HashMap::new()
        }
    }

    pub fn create_scene(&mut self) -> SceneId {
        let scene: Scene = Scene::new();
        let scene_id: SceneId = Uuid::new_v4();

        self.scenes.insert(scene_id, scene);

        scene_id
    }

    pub fn get_scene(&mut self, scene_id: SceneId) -> Option<&mut Scene> {
        self.scenes.get_mut(&scene_id)
    }

    pub fn render(&self, scene_id: SceneId, camera_id: CameraID) -> Image {
        let scene: &Scene = self.scenes.get(&scene_id).unwrap();
        let camera: &Camera = scene.get_camera(camera_id).unwrap();

        let camera_planes: HashMap<PlaneDirection, Plane> = camera.create_planes();

        render(camera_planes, camera.focal_length(), scene.get_all_meshes())
    }
}
