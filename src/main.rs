extern crate sdl2;

use std::time::{Duration, Instant};

use crate::rendering_engine::engine::compositor::Image;
use crate::rendering_engine::scene::camera::display::Display;
use crate::rendering_engine::scene::camera::Camera;
use crate::rendering_engine::scene::model::color::Color as RenderingColor;
use crate::rendering_engine::scene::model_3d::point::Point as Point3D;
use crate::rendering_engine::scene::model_3d::vector::Vector;
use crate::rendering_engine::scene::Scene;
use crate::rendering_engine::{RenderingEngine, SceneId};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::WindowCanvas;
use sdl2::video::Window;
use sdl2::{Sdl, VideoSubsystem};

mod rendering_engine;

const MOVE_STEP: f32 = 10.0;

pub fn main() {
    const WIDTH: usize = 800;
    const HEIGHT: usize = 800;

    let mut renderer: RenderingEngine = RenderingEngine::new();
    let scene_id: SceneId = renderer.create_scene();
    let scene: &mut Scene = renderer.get_scene(scene_id).unwrap();

    let camera_id = scene.add_camera(
        Camera::new(
            (((WIDTH + HEIGHT) / 2) * 2) as f32,
            Point3D { x: 0.0, y: 0.0, z: 0.0 },
            0.0,
            0.0,
            0.0,
            Display::new(WIDTH, HEIGHT)
        )
    );

    scene.add_cube(Point3D { x: 0.0, y: 0.0, z: 300.0 }, 100.0, 100.0, 100.0, RenderingColor::new(255, 0, 0, 0.2));
    scene.add_cube(Point3D { x: 0.0, y: 0.0, z: 800.0 }, 100.0, 100.0, 100.0, RenderingColor::new(255, 0, 0, 0.5));
    // scene.add_cube(Point3D { x: 120.0, y: 0.0, z: 300.0 }, 100.0, 100.0, 100.0, RenderingColor::new(255, 0, 0, 1.0));


    let sdl_context: Sdl = sdl2::init().unwrap();
    let video_subsystem: VideoSubsystem = sdl_context.video().unwrap();

    let window: Window = video_subsystem.window("rust-sdl2 demo", WIDTH as u32, HEIGHT as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas: WindowCanvas = window.into_canvas().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::W), .. } => {
                    renderer.reposition_camera(scene_id, camera_id, Vector { x: 0.0, y: 0.0, z: -MOVE_STEP });
                }
                Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                    renderer.reposition_camera(scene_id, camera_id, Vector { x: 0.0, y: 0.0, z: MOVE_STEP });
                }
                Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                    renderer.reposition_camera(scene_id, camera_id, Vector { x: -MOVE_STEP, y: 0.0, z: 0.0 });
                }
                Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                    renderer.reposition_camera(scene_id, camera_id, Vector { x: MOVE_STEP, y: 0.0, z: 0.0 });
                }
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    renderer.rotate_camera(scene_id, camera_id, &Vector { x: 0.0, y: -0.5, z: 0.0 });
                }
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    renderer.rotate_camera(scene_id, camera_id, &Vector { x: 0.0, y: 0.5, z: 0.0 });
                }
                _ => {}
            }
        }

        let before = Instant::now();
        let image: Image = renderer.render(scene_id, camera_id, RenderingColor::new(255, 255, 255, 1.0));
        render(image, &mut canvas);
        println!("Elapsed time: {:.2?}", before.elapsed());

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn render(image: Image, canvas: &mut WindowCanvas) {
    for (row, row_pixels) in image.iter().enumerate() {
        for (col, pixel) in row_pixels.iter().enumerate() {
            let x: i32 = col as i32;
            let y: i32 = row as i32;

            canvas.set_draw_color(Color::RGB(pixel.r, pixel.g, pixel.b));
            canvas.draw_point(Point::new(x, y)).unwrap();
        }
    }

    canvas.present();
}
