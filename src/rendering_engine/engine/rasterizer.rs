use serde::{Deserialize, Serialize};

use crate::rendering_engine::engine::depth_pixel::DepthPixel;
use crate::rendering_engine::engine::pixel::Pixel;
use crate::rendering_engine::scene::model::color::Color;
use crate::rendering_engine::scene::model_2d::point::Point as Point2D;
use crate::rendering_engine::scene::model_2d::triangle::Triangle as Triangle2D;
use crate::rendering_engine::scene::model_3d::triangle::Triangle as Triangle3D;

pub type DepthPixelBuffer = Vec<Vec<DepthPixel>>;

#[derive(Serialize, Deserialize)]
pub struct ZBuffer {
    pub buffer: DepthPixelBuffer,
    pub x: usize,
    pub y: usize,
}

pub fn rasterize(triangle2d: &Triangle2D, triangle3d: &Triangle3D) -> ZBuffer {
    let (x, y, width, height) = create_bounding_box(triangle2d);
    let buffer_width: usize = (width.ceil() + 1.0) as usize;
    let buffer_height: usize = (height.ceil() + 1.0) as usize;

    let mut pixel_buffer: DepthPixelBuffer =
        vec![
            vec![DepthPixel::default(); buffer_width];
            buffer_height
        ];

    let relative_point0 = Point2D {
        x: triangle2d.vertices[0].x - x,
        y: triangle2d.vertices[0].y - y,
    };

    let relative_point1 = Point2D {
        x: triangle2d.vertices[1].x - x,
        y: triangle2d.vertices[1].y - y,
    };

    let relative_point2 = Point2D {
        x: triangle2d.vertices[2].x - x,
        y: triangle2d.vertices[2].y - y,
    };

    draw_line(
        relative_point0, triangle3d.vertices()[0].z,
        relative_point1, triangle3d.vertices()[1].z,
        triangle3d.color(),
        &mut pixel_buffer
    );

    draw_line(
        relative_point1, triangle3d.vertices()[1].z,
        relative_point2, triangle3d.vertices()[2].z,
        triangle3d.color(),
        &mut pixel_buffer
    );

    draw_line(
        relative_point2, triangle3d.vertices()[2].z,
        relative_point0, triangle3d.vertices()[0].z,
        triangle3d.color(),
        &mut pixel_buffer
    );

    ZBuffer {
        buffer: pixel_buffer,
        x: x.floor() as usize,
        y: y.floor() as usize
    }
}

/** brightness is a value from 0 to 1 */
fn set_pixel(col: isize, row: isize, color: Color, pixel_buffer: &mut DepthPixelBuffer) {
    let col: isize = (pixel_buffer[0].len() / 2) as isize + col;
    let row: isize = (pixel_buffer.len() / 2) as isize - row;

    let col: usize = col as usize;
    let row: usize = row as usize;

    if row >= pixel_buffer.len() || col >= pixel_buffer[0].len() {
        return;
    }

    pixel_buffer[row][col] = DepthPixel {
        pixel: Pixel::new(color.r, color.g, color.b, color.a),
        depth: 1.0 /* @FIXME */
    }
}

fn draw_line(mut point0: Point2D, point0depth: f32,
             mut point1: Point2D, point1depth: f32,
             color: Color,
             pixel_buffer: &mut DepthPixelBuffer) {
    let dx = point1.x - point0.x;
    let dy = point1.y - point0.y;

    if dx > dy {
        let m = dy / dx;

        for x in point0.x as isize..=point1.x as isize {
            let y = if m == 0.0 {
                0.0
            } else {
                m * (x as f32 - point0.x) + point0.y
            };

            set_pixel(x, y as isize, color, pixel_buffer);
        }
    } else {
        let m = dx / dy;

        for y in point0.y as isize..=point1.y as isize {
            let x = if m == 0.0 {
                0.0
            } else {
                (-y as f32 - m*point0.x + point0.y) / m
            };

            set_pixel(x as isize, y, color, pixel_buffer);
        }
    }
}

fn create_bounding_box(triangle2d: &Triangle2D) -> (f32, f32, f32, f32) {
    let mut min_x: f32 = triangle2d.vertices[0].x;
    let mut min_y: f32 = triangle2d.vertices[0].y;
    let mut max_x: f32 = triangle2d.vertices[0].x;
    let mut max_y: f32 = triangle2d.vertices[0].y;

    for point in triangle2d.vertices.iter() {
        if point.x < min_x {
            min_x = point.x;
        }
        if point.x > max_x {
            max_x = point.x;
        }

        if point.y < min_y {
            min_y = point.y;
        }
        if point.y > max_y {
            max_y = point.y;
        }
    }

    (min_x, min_y, max_x - min_x, max_y - min_y)
}
