use std::collections::{HashMap, HashSet};
use std::thread::scope;
use js_sys::Array;
use js_sys::Math::pow;
use wasm_bindgen::prelude::wasm_bindgen;
use once_cell::unsync::Lazy;
use uuid::Uuid;
use wasm_bindgen::convert::js_value_vector_from_abi;
use wasm_bindgen::JsValue;
use crate::renderer::{Renderer, SceneId};
use crate::renderer::scene::camera::Camera;
use crate::renderer::scene::camera::display::Display;
use crate::renderer::scene::model_2d::point::{Point as Point2D};
use crate::renderer::scene::model_3d::point::{Point as Point3D};
use crate::renderer::scene::{CameraID, MeshID, Scene};
use crate::renderer::scene::model_2d::triangle::Triangle as Triangle2D;
use crate::renderer::scene::model_3d::triangle::Triangle as Triangle3D;
use crate::renderer::scene::model_3d::plane::Plane;
use crate::renderer::scene::model_3d::plane_direction::PlaneDirection;
use crate::renderer::scene::model_3d::vector::Vector;

mod renderer;

static mut RENDERERS: Vec<Renderer> = vec![];

#[wasm_bindgen]
pub unsafe fn init_renderer() {
    RENDERERS.push(Renderer::new());
}

#[wasm_bindgen]
pub unsafe fn init_scene() -> String {
    RENDERERS[0].create_scene().to_string()
}

#[wasm_bindgen]
pub unsafe fn add_camera(scene_id: String) -> String {
    let scene_id: Uuid = Uuid::parse_str(scene_id.as_str()).unwrap();

    let width: usize = 800;
    let height: usize = 800;

    RENDERERS[0].get_scene(scene_id).unwrap().add_camera(Camera::new(
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
pub unsafe fn get_mesh(scene_id: String, mesh_id: String) -> JsValue {
    let scene_id: Uuid = Uuid::parse_str(scene_id.as_str()).unwrap();
    let mesh_id: Uuid = Uuid::parse_str(mesh_id.as_str()).unwrap();

    serde_wasm_bindgen::to_value(&RENDERERS[0].get_scene(scene_id).unwrap().get_mesh(mesh_id).unwrap()).unwrap()
}

#[wasm_bindgen]
pub unsafe fn triangulate_mesh(scene_id: String, mesh_id: String) -> JsValue {
    let scene_id: Uuid = Uuid::parse_str(scene_id.as_str()).unwrap();
    let mesh_id: Uuid = Uuid::parse_str(mesh_id.as_str()).unwrap();

    serde_wasm_bindgen::to_value(&RENDERERS[0].get_scene(scene_id).unwrap().get_mesh(mesh_id).unwrap().triangulate()).unwrap()
}

#[wasm_bindgen]
pub unsafe fn add_mesh(scene_id: String) -> String {
    let scene_id: Uuid = Uuid::parse_str(scene_id.as_str()).unwrap();

    RENDERERS[0].get_scene(scene_id).unwrap().add_mesh(
        vec![
            Point3D { x: -150.0, y: -50.0, z: 250.0 }, /* bottom left */
            Point3D { x: -150.0, y: 50.0, z: 250.0 }, /* top left */
            Point3D { x: -50.0, y: 50.0, z: 250.0 }, /* top right */
        ],
        vec![
            [0, 1, 2]
        ]
    ).to_string()
}

#[wasm_bindgen]
pub unsafe fn add_cube(scene_id: String, x: f32, y: f32, z: f32, width: f32, height: f32, length: f32) -> String {
    let scene_id: Uuid = Uuid::parse_str(scene_id.as_str()).unwrap();

    RENDERERS[0].get_scene(scene_id).unwrap().add_cube(
        Point3D { x, y, z },
        width, height, length
    ).to_string()
}

#[wasm_bindgen]
pub unsafe fn render(scene_id: String, camera_id: String) -> String {
    let scene_id: Uuid = Uuid::parse_str(scene_id.as_str()).unwrap();
    let camera_id: Uuid = Uuid::parse_str(camera_id.as_str()).unwrap();

    let triangles: Vec<Triangle2D> = RENDERERS[0].get_scene(scene_id).unwrap().render(camera_id).unwrap();

    serde_json::to_string(&triangles).unwrap()
}