use uuid::Uuid;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

use crate::rendering_engine::scene::camera::display::Display;
use crate::rendering_engine::scene::camera::Camera;
use crate::rendering_engine::scene::model::color::Color;
use crate::rendering_engine::scene::model_3d::face::Face;
use crate::rendering_engine::scene::model_3d::point::Point as Point3D;
use crate::rendering_engine::scene::model_3d::vector::Vector as Vector3D;
use crate::rendering_engine::RenderingEngine;

mod rendering_engine;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

static mut RENDERING_ENGINES: Vec<RenderingEngine> = vec![];

#[wasm_bindgen]
pub unsafe fn init_renderer() {
    RENDERING_ENGINES.push(RenderingEngine::new());
}

#[wasm_bindgen]
pub unsafe fn init_scene() -> String {
    RENDERING_ENGINES[0].create_scene().to_string()
}

#[wasm_bindgen]
pub unsafe fn add_camera(scene_id: String) -> String {
    let scene_id: Uuid = Uuid::parse_str(scene_id.as_str()).unwrap();

    let width: usize = 800;
    let height: usize = 800;

    RENDERING_ENGINES[0]
        .get_scene(scene_id).unwrap()
        .add_camera(Camera::new(
            (width as f32) * 2.0,
            Point3D { x: 0.0, y: 0.0, z: 0.0 },
            0.0,
            0.0,
            0.0,
            Display::new(width, height)
        ))
        .to_string()
}

#[wasm_bindgen]
pub unsafe fn add_mesh(scene_id: String) -> String {
    let scene_id: Uuid = Uuid::parse_str(scene_id.as_str()).unwrap();

    RENDERING_ENGINES[0]
        .get_scene(scene_id).unwrap()
        .add_mesh(
            vec![
                Point3D { x: 0.0, y: 0.0, z: 250.0 }, /* bottom left */
                Point3D { x: 0.0, y: 50.0, z: 250.0 }, /* top left */
                Point3D { x: 50.0, y: 0.0, z: 250.0 }, /* top right */
            ],
            vec![
                Face::new([0, 1, 2], Color::new(255, 0, 0, 0.5))
            ]
        ).to_string()
}

#[wasm_bindgen]
pub unsafe fn add_cube(scene_id: String, position: JsValue, width: f32, height: f32, length: f32) -> String {
    let scene_id: Uuid = Uuid::parse_str(scene_id.as_str()).unwrap();

    let position: Point3D = serde_wasm_bindgen::from_value(position).unwrap();

    RENDERING_ENGINES[0].get_scene(scene_id).unwrap().add_cube(
        position,
        width, height, length,
        Color::new(255, 0, 0, 0.5)
    ).to_string()
}

#[wasm_bindgen]
pub unsafe fn move_camera(scene_id: String, camera_id: String, delta: JsValue) {
    let scene_id: Uuid = Uuid::parse_str(scene_id.as_str()).unwrap();
    let camera_id: Uuid = Uuid::parse_str(camera_id.as_str()).unwrap();

    let delta: Vector3D = serde_wasm_bindgen::from_value(delta).unwrap();

    RENDERING_ENGINES[0]
        .get_scene(scene_id).unwrap()
        .get_camera_mut(camera_id).unwrap()
        .reposition(delta);
}

#[wasm_bindgen]
pub unsafe fn get_camera(scene_id: String, camera_id: String) -> JsValue {
    let scene_id: Uuid = Uuid::parse_str(scene_id.as_str()).unwrap();
    let camera_id: Uuid = Uuid::parse_str(camera_id.as_str()).unwrap();

    serde_wasm_bindgen::to_value(
        RENDERING_ENGINES[0]
            .get_scene(scene_id).unwrap()
            .get_camera_mut(camera_id).unwrap()
    ).unwrap()
}

#[wasm_bindgen]
pub unsafe fn rotate_camera(scene_id: String, camera_id: String, delta: JsValue) {
    let scene_id: Uuid = Uuid::parse_str(scene_id.as_str()).unwrap();
    let camera_id: Uuid = Uuid::parse_str(camera_id.as_str()).unwrap();

    let delta: Vector3D = serde_wasm_bindgen::from_value(delta).unwrap();

    RENDERING_ENGINES[0]
        .get_scene(scene_id).unwrap()
        .get_camera_mut(camera_id).unwrap()
        .rotate(&delta);
}

#[wasm_bindgen]
pub unsafe fn move_camera_focal_length(scene_id: String, camera_id: String, delta: f32) {
    let scene_id: Uuid = Uuid::parse_str(scene_id.as_str()).unwrap();
    let camera_id: Uuid = Uuid::parse_str(camera_id.as_str()).unwrap();

    RENDERING_ENGINES[0]
        .get_scene(scene_id).unwrap()
        .get_camera_mut(camera_id).unwrap()
        .move_focal_length(delta);
}

// #[wasm_bindgen]
// pub unsafe fn render(scene_id: String, camera_id: String) -> JsValue {
//     let scene_id: Uuid = Uuid::parse_str(scene_id.as_str()).unwrap();
//     let camera_id: Uuid = Uuid::parse_str(camera_id.as_str()).unwrap();
//
//     let triangles: Vec<Triangle2D> = RENDERING_ENGINES[0]
//         .get_scene(scene_id).unwrap()
//         .render(camera_id).unwrap();
//
//     serde_wasm_bindgen::to_value(&triangles).unwrap()
// }
#[wasm_bindgen]
pub unsafe fn render_new(scene_id: String, camera_id: String) -> JsValue {
    let scene_id: Uuid = Uuid::parse_str(scene_id.as_str()).unwrap();
    let camera_id: Uuid = Uuid::parse_str(camera_id.as_str()).unwrap();

    serde_wasm_bindgen::to_value(&RENDERING_ENGINES[0].render(scene_id, camera_id, Color::new(255, 255, 255, 1.0))).unwrap()
}
