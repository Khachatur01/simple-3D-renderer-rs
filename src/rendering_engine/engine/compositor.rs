use crate::rendering_engine::engine::pixel::Pixel;
use crate::rendering_engine::engine::rasterizer::ZBuffer;
use crate::rendering_engine::scene::camera::display::Display;
use crate::rendering_engine::scene::model::color::Color;

pub type Image = Vec<Vec<Pixel>>;

pub fn composite(z_buffers: &Vec<ZBuffer>, display: &Display, background_color: Color) -> Image {
    let default_pixel: Pixel = Pixel::new(
        background_color.r,
        background_color.g,
        background_color.b,
        background_color.a,
    );

    let mut image = vec![
        vec![default_pixel; display.width];
        display.height
    ];

    for (row, row_pixels) in image.iter_mut().enumerate() {
        for (col, pixel) in row_pixels.iter_mut().enumerate() {
            if let Some(result_pixel) = composite_pixel(row, col, &z_buffers, default_pixel.color) {
                pixel.color = result_pixel.color.clone();
            }
        }
    }

    image
}


/* @FIXME refactor */
fn composite_pixel(row: usize, col: usize, z_buffers: &Vec<ZBuffer>, background_color: Color) -> Option<Pixel> {
    for z_buffer in z_buffers {
        let row: isize = row as isize - z_buffer.y as isize;
        let col: isize = col as isize - z_buffer.x as isize;

        if row < 0 || row >= z_buffer.buffer.len() as isize ||
            col < 0 || col >= z_buffer.buffer[0].len() as isize {
            continue;
        }

        let depth_pixel = z_buffer.buffer
            .get(row as usize)
            .map(|row_pixels| row_pixels.get(col as usize));

        if let Some(Some(depth_pixel)) = depth_pixel {
            if depth_pixel.pixel.color.a != 0 {
                let mut depth_pixel: Pixel = depth_pixel.pixel.clone();

                let alpha = (depth_pixel.color.a as f32) / 256.0;

                depth_pixel.color.r = ((1.0 - alpha) * background_color.r as f32 + alpha * depth_pixel.color.r as f32) as u8;
                depth_pixel.color.g = ((1.0 - alpha) * background_color.g as f32 + alpha * depth_pixel.color.g as f32) as u8;
                depth_pixel.color.b = ((1.0 - alpha) * background_color.b as f32 + alpha * depth_pixel.color.b as f32) as u8;

                return Some(depth_pixel);
            }
        }
    }

    None
}
