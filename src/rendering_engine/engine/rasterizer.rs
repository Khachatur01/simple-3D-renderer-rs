use std::ops::RangeInclusive;

use serde::{Deserialize, Serialize};

use crate::log;
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
fn set_pixel(x: usize, y: usize, color: Color, pixel_buffer: &mut DepthPixelBuffer) {
    if y >= pixel_buffer.len() || x >= pixel_buffer[0].len() {
        log(format!("{y}, {x} is out of range").as_str());
        return;
    }
    // log(format!("{y}, {x} is in range").as_str());

    pixel_buffer[y][x] = DepthPixel {
        pixel: Pixel::new(color.r, color.g, color.b, color.a),
        depth: 1.0 /* @FIXME */
    }
}

fn i_part(number: f32) -> f32 {
    number.ceil()
}
fn f_part(number: f32) -> f32 {
    number - i_part(number)
}
fn rf_part(number: f32) -> f32 {
    1.0 - f_part(number)
}

fn draw_line(mut point0: Point2D, point0depth: f32,
             mut point1: Point2D, point1depth: f32,
             color: Color,
             pixel_buffer: &mut DepthPixelBuffer) {

    let steep: bool = (point1.y - point0.y).abs() > (point1.x - point0.x).abs();
    // const steep = Math.abs(y1 - y0) > Math.abs(x1 - x0);


    log(serde_json::to_string(&point0).unwrap().as_str());
    log(serde_json::to_string(&point1).unwrap().as_str());

    if steep {
        (point0.x, point0.y) = (point0.y, point0.x);
        (point1.x, point1.y) = (point1.y, point1.x);
        log("steep");
    }
    // if (steep) {
    //     [x0, y0] = [y0, x0];
    //     [x1, y1] = [y1, x1];
    // }

    if point0.x > point1.x {
        (point0.x, point1.x) = (point1.x, point0.x);
        (point0.y, point1.y) = (point1.y, point0.y);
        log("steep2");
    }
    // if (x0 > x1) {
    //     [x0, x1] = [x1, x0];
    //     [y0, y1] = [y1, y0];
    // }

    log(serde_json::to_string(&point0).unwrap().as_str());
    log(serde_json::to_string(&point1).unwrap().as_str());

    let dx: f32 = point1.x - point0.x;
    let dy: f32 = point1.y - point0.y;
    let gradient: f32 = if dx == 0.0 || dy == 0.0 {
        1.0
    } else {
        dy / dx
    };

    // const dx = x1 - x0;
    // const dy = y1 - y0;
    // const gradient = dy / dx || 1;

    let range: RangeInclusive<usize> = point0.x.floor() as usize..=point1.x.floor() as usize;
    let mut intersect_y: f32 = point0.y;
    log(serde_json::to_string(&range).unwrap().as_str());

    // let xpxl1 = Math.floor(x0);
    // let xpxl2 = Math.floor(x1);
    // let intersectY = y0;

    if steep {
        for x in range {
            let mut color: Color = color.clone();

            color.a = (rf_part(intersect_y) * 256.0) as u8;
            set_pixel(intersect_y.floor() as usize, x, color, pixel_buffer);

            color.a = (f_part(intersect_y) * 256.0) as u8;
            set_pixel((intersect_y.floor() - 1.0) as usize, x, color, pixel_buffer);

            intersect_y += gradient;
        }
    } else {
        for x in range {
            let mut color: Color = color.clone();

            color.a = (rf_part(intersect_y) * 256.0) as u8;
            set_pixel(x, intersect_y.floor() as usize, color, pixel_buffer);

            color.a = (f_part(intersect_y) * 256.0) as u8;
            set_pixel(x, (intersect_y.floor() - 1.0) as usize, color, pixel_buffer);

            intersect_y += gradient;
        }
    }
    //
    // if (steep) {
    //     for (let x = xpxl1; x <= xpxl2; x++) {
    //         drawPixel(ctx, Math.floor(intersectY), x, rfPartOfNumber(intersectY));
    //         drawPixel(ctx, Math.floor(intersectY) - 1, x, fPartOfNumber(intersectY));
    //         intersectY += gradient;
    //     }
    // } else {
    //     for (let x = xpxl1; x <= xpxl2; x++) {
    //         drawPixel(ctx, x, Math.floor(intersectY), rfPartOfNumber(intersectY));
    //         drawPixel(ctx, x, Math.floor(intersectY) - 1, fPartOfNumber(intersectY));
    //         intersectY += gradient;
    //     }
    // }

    log("\n\n\n");
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
