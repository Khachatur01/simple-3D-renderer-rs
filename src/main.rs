extern crate sdl2;

use std::time::Duration;

use sdl2::{Sdl, VideoSubsystem};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::WindowCanvas;
use sdl2::video::Window;

use crate::rendering_engine::engine::compositor::Image;
use crate::rendering_engine::RenderingEngine;
use crate::rendering_engine::scene::camera::Camera;
use crate::rendering_engine::scene::camera::display::Display;
use crate::rendering_engine::scene::model::color::Color as RenderingColor;
use crate::rendering_engine::scene::model_3d::point::Point as Point3D;

mod rendering_engine;

pub fn main() {
    const WIDTH: usize = 800;
    const HEIGHT: usize = 800;

    let mut renderer: RenderingEngine = RenderingEngine::new();
    let scene_id = renderer.create_scene();
    let scene = renderer.get_scene(scene_id).unwrap();

    let camera_id = scene.add_camera(
        Camera::new(
            800.0 * 2.0,
            Point3D { x: 0.0, y: 0.0, z: 0.0 },
            0.0,
            0.0,
            0.0,
            Display::new(WIDTH, HEIGHT)
        )
    );

    let camera: &mut Camera = scene.get_camera_mut(camera_id).unwrap();

    scene.add_cube(Point3D { x: 0.0, y: 0.0, z: 300.0 }, 100.0, 100.0, 100.0, RenderingColor::new(255, 0, 0, 255));
    scene.add_cube(Point3D { x: 125.0, y: 0.0, z: 300.0 }, 100.0, 100.0, 100.0, RenderingColor::new(255, 0, 0, 255));

    let sdl_context: Sdl = sdl2::init().unwrap();
    let video_subsystem: VideoSubsystem = sdl_context.video().unwrap();

    let window: Window = video_subsystem.window("rust-sdl2 demo", WIDTH as u32, HEIGHT as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas: WindowCanvas = window.into_canvas().build().unwrap();

    let image: Image = renderer.render(scene_id, camera_id, RenderingColor::new(255, 255, 255, 255));
    render(image, &mut canvas);

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                // Event::KeyDown { keycode: Some(Keycode::W), .. } => {
                //     camera.reposition(Vector { x: 0.0, y: 0.0, z: 10.0 });
                // }
                // Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                //     camera.reposition(Vector { x: 0.0, y: 0.0, z: -10.0 });
                // }
                _ => {}
            }
        }

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn render(image: Image, canvas: &mut WindowCanvas) {
    for (row, row_pixels) in image.iter().enumerate() {
        for (col, pixel) in row_pixels.iter().enumerate() {
            if pixel.color.a == 0 {
                continue;
            }

            let x: i32 = col as i32;
            let y: i32 = row as i32;

            canvas.set_draw_color(Color::RGB(pixel.color.r, pixel.color.g, pixel.color.b));
            canvas.draw_point(Point::new(x, y)).unwrap();
        }
    }
    canvas.present();
}
