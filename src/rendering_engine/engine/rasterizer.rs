use line_drawing::XiaolinWu;
use serde::{Deserialize, Serialize};

use crate::rendering_engine::engine::depth_pixel::DepthPixel;
use crate::rendering_engine::engine::pixel::Pixel;
use crate::rendering_engine::engine::z_buffered_triangle::{ZBufferedTriangle, ZBufferedVertex};
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
    pub x: usize,
    pub y: usize,
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

    let relative_vertex0 = ZBufferedVertex {
        x: z_buffered_triangle.vertices[0].x - x,
        y: z_buffered_triangle.vertices[0].y - y,
        distance: z_buffered_triangle.vertices[0].distance
    };

    let relative_vertex1 = ZBufferedVertex {
        x: z_buffered_triangle.vertices[1].x - x,
        y: z_buffered_triangle.vertices[1].y - y,
        distance: z_buffered_triangle.vertices[1].distance
    };

    let relative_vertex2 = ZBufferedVertex {
        x: z_buffered_triangle.vertices[2].x - x,
        y: z_buffered_triangle.vertices[2].y - y,
        distance: z_buffered_triangle.vertices[2].distance
    };

    draw_line(
        relative_vertex0,
        relative_vertex1,
        z_buffered_triangle.color,
        &mut pixel_buffer
    );

    draw_line(
        relative_vertex1,
        relative_vertex2,
        z_buffered_triangle.color,
        &mut pixel_buffer
    );

    draw_line(
        relative_vertex2,
        relative_vertex0,
        z_buffered_triangle.color,
        &mut pixel_buffer
    );

    ZBuffer {
        buffer: pixel_buffer,
        x: x.floor() as usize,
        y: y.floor() as usize
    }
}

fn draw_line(point0: ZBufferedVertex,
             point1: ZBufferedVertex,
             color: Color,
             pixel_buffer: &mut DepthPixelBuffer) {

    for ((x, y), value) in XiaolinWu::<f32, isize>::new((point0.x, point0.y), (point1.x, point1.y)) {
        set_pixel(x as usize, y as usize,
                  Color::new(color.r, color.g, color.b, (color.a as f32 * value) as u8),
                  pixel_buffer
        );
    }
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
fn set_pixel(col: usize, row: usize, color: Color, pixel_buffer: &mut DepthPixelBuffer) {
    if row >= pixel_buffer.len() || col >= pixel_buffer[0].len() {
        return;
    }

    pixel_buffer[row][col] = DepthPixel {
        pixel: Pixel::new(color.r, color.g, color.b, color.a),
        depth: 1.0 /* @FIXME */
    }
}