use std::collections::HashMap;

use uuid::Uuid;

use scene::Scene;

pub mod scene;

pub type SceneId = Uuid;

pub struct Renderer {
    scenes: HashMap<SceneId, Scene>
}

impl Renderer {
    pub fn new() -> Renderer {
        Renderer {
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
}
