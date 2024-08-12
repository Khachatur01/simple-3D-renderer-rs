use std::collections::HashMap;

use uuid::Uuid;

use scene::Scene;

use crate::rendering_engine::engine::compositor::Image;
use crate::rendering_engine::engine::renderer::render;
use crate::rendering_engine::scene::camera::Camera;
use crate::rendering_engine::scene::model::color::Color;
use crate::rendering_engine::scene::model_3d::vector::Vector;
use crate::rendering_engine::scene::CameraID;

pub mod scene;
pub mod engine;

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

    pub fn reposition_camera(&mut self, scene_id: SceneId, camera_id: CameraID, delta: Vector) {
        self.scenes
            .get_mut(&scene_id).unwrap()
            .get_camera_mut(camera_id).unwrap()
            .reposition(delta);
    }

    pub fn rotate_camera(&mut self, scene_id: SceneId, camera_id: CameraID, delta: &Vector) {
        self.scenes
            .get_mut(&scene_id).unwrap()
            .get_camera_mut(camera_id).unwrap()
            .rotate(delta);
    }

    pub fn move_camera_focal_length(&mut self, scene_id: SceneId, camera_id: CameraID, delta: f32) {
        self.scenes
            .get_mut(&scene_id).unwrap()
            .get_camera_mut(camera_id).unwrap()
            .move_focal_length(delta);
    }

    pub fn render(&self, scene_id: SceneId, camera_id: CameraID, background_color: Color) -> Image {
        let scene: &Scene = self.scenes.get(&scene_id).unwrap();
        let camera: &Camera = scene.get_camera(camera_id).unwrap();

        render(camera, scene.get_all_meshes(), background_color)
    }
}
