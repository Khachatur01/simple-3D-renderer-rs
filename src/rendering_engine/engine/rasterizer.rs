use line_drawing::XiaolinWu;
use serde::{Deserialize, Serialize};

use crate::rendering_engine::engine::model::depth_pixel::DepthPixel;
use crate::rendering_engine::engine::model::z_buffered_triangle::{ZBufferedTriangle, ZBufferedVertex, ZBufferedVertices};
use crate::rendering_engine::scene::model::color::Color;

pub type DepthPixelBuffer = Vec<Vec<DepthPixel>>;

/**
    Values:
    x, y, width, height
 */
type BoundingBox = (f32, f32, f32, f32);

#[derive(Serialize, Deserialize)]
pub struct ZBuffer {
    pub buffer: DepthPixelBuffer,
    pub x: isize,
    pub y: isize,
}

pub fn rasterize(z_buffered_triangle: &ZBufferedTriangle) -> ZBuffer {
    let (x, y, width, height) = create_bounding_box(z_buffered_triangle);
    let buffer_width: usize = width.ceil() as usize;
    let buffer_height: usize = height.ceil() as usize;

    let mut pixel_buffer: DepthPixelBuffer =
        vec![
            vec![DepthPixel::default(); buffer_width + 1];
            buffer_height + 1
        ];

    let relative_vertices: ZBufferedVertices = z_buffered_triangle.vertices
        .map(|z_buffered_vertex: ZBufferedVertex| {
            ZBufferedVertex {
                x: z_buffered_vertex.x - x,
                y: z_buffered_vertex.y - y,
                distance: z_buffered_vertex.distance
            }
        });

    for i in 0..3 {
        let current: usize = i;
        let next: usize = if i == 2 {
            0
        } else {
            i + 1
        };

        draw_line(
            relative_vertices[current],
            relative_vertices[next],
            z_buffered_triangle.color,
            &mut pixel_buffer
        );
    }

    let fill_lines: Vec<(ZBufferedVertex, ZBufferedVertex)> = get_fill_lines(&mut pixel_buffer);
    fill(z_buffered_triangle.color, fill_lines, &mut pixel_buffer);

    ZBuffer {
        buffer: pixel_buffer,
        x: x.floor() as isize,
        y: y.floor() as isize
    }
}

#[inline]
fn draw_line(point0: ZBufferedVertex,
             point1: ZBufferedVertex,
             color: Color,
             pixel_buffer: &mut DepthPixelBuffer) {

    let line: XiaolinWu<f32, isize> = XiaolinWu::new((point0.x, point0.y), (point1.x, point1.y));

    let line_length: f32 = ((point0.x - point1.x).powi(2) + (point0.y - point1.y).powi(2)).sqrt();

    let depth_step: f32 = (point1.distance - point0.distance) / line_length;

    for ((x, y), opacity) in line {
        set_pixel(x as usize, y as usize,
                  point0.distance + depth_step,
                  Color::new(color.r, color.g, color.b, color.a * opacity),
                  pixel_buffer
        );
    }
}

fn get_fill_lines(pixel_buffer: &mut DepthPixelBuffer) -> Vec<(ZBufferedVertex, ZBufferedVertex)> {
    pixel_buffer
        .iter()
        .enumerate()
        .map(|(row, row_pixels)| {
            get_fill_line(row, row_pixels)
        })
        .collect()
}

fn fill(
    color: Color,
    fill_lines: Vec<(ZBufferedVertex, ZBufferedVertex)>,
    pixel_buffer: &mut DepthPixelBuffer) {

    fill_lines.iter().for_each(|fill_line: &(ZBufferedVertex, ZBufferedVertex)| {
        draw_line(fill_line.0, fill_line.1, color, pixel_buffer);
    });
}

fn get_fill_line(row: usize, row_pixels: &Vec<DepthPixel>) -> (ZBufferedVertex, ZBufferedVertex) {
    let mut left: &DepthPixel = row_pixels.first().unwrap();
    let mut right: &DepthPixel= row_pixels.last().unwrap();

    let mut left_column: usize = 0;
    let mut right_column: usize = row_pixels.len() - 1;

    for (col, col_pixel) in row_pixels.iter().enumerate() {
        if col_pixel.color.a < left.color.a {
            break;
        }

        if col_pixel.color.a > left.color.a {
            left = col_pixel;
            left_column = col;
        }
    }

    for (col, col_pixel) in row_pixels.iter().rev().enumerate() {
        if col_pixel.color.a < right.color.a {
            break;
        }

        if col_pixel.color.a > right.color.a {
            right = col_pixel;
            right_column = row_pixels.len() - col - 1;
        }
    }

    let left: ZBufferedVertex = ZBufferedVertex {
        x: left_column as f32,
        y: row as f32,
        distance: left.depth
    };

    let right: ZBufferedVertex = ZBufferedVertex {
        x: right_column as f32,
        y: row as f32,
        distance: right.depth
    };

    (left, right)
}

fn create_bounding_box(z_buffered_triangle: &ZBufferedTriangle) -> (f32, f32, f32, f32) {
    let bounding_box: BoundingBox = z_buffered_triangle.vertices
        .iter()
        .fold((f32::MAX, f32::MAX, f32::MIN, f32::MIN), |bounding_box: BoundingBox, vertex: &ZBufferedVertex| {
            (
                f32::min(vertex.x, bounding_box.0),
                f32::min(vertex.y, bounding_box.1),
                f32::max(vertex.x, bounding_box.2),
                f32::max(vertex.y, bounding_box.3),
            )
        });

    (
        /* x */         /* y */
        bounding_box.0, bounding_box.1,
        /* width */
        bounding_box.2 - bounding_box.0,
        /* height */
        bounding_box.3 - bounding_box.1
    )
}


/** brightness is a value from 0 to 1 */
#[inline]
fn set_pixel(col: usize, row: usize, depth: f32, color: Color, pixel_buffer: &mut DepthPixelBuffer) {
    if row >= pixel_buffer.len() || col >= pixel_buffer[0].len() {
        return;
    }

    pixel_buffer[row][col] = DepthPixel {
        color: Color::new(color.r, color.g, color.b, color.a),
        depth
    }
}