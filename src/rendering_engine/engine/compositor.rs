use crate::rendering_engine::engine::model::pixel::Pixel;
use crate::rendering_engine::engine::rasterizer::ZBuffer;
use crate::rendering_engine::scene::camera::display::Display;
use crate::rendering_engine::scene::model::color::Color;

pub type Image = Vec<Vec<Pixel>>;

pub fn composite(z_buffers: &Vec<ZBuffer>, display: &Display, background_color: Color) -> Image {
    let default_pixel: Pixel = Pixel::new(
        background_color.r,
        background_color.g,
        background_color.b
    );

    let mut image: Image = vec![
        vec![default_pixel; display.width];
        display.height
    ];

    for (row, row_pixels) in image.iter_mut().enumerate() {
        for (col, pixel) in row_pixels.iter_mut().enumerate() {
            let composed_pixel: Pixel = blend_pixel(row, col, &z_buffers, background_color);
            pixel.r = composed_pixel.r;
            pixel.g = composed_pixel.g;
            pixel.b = composed_pixel.b;
        }
    }

    image
}

fn blend_pixel(row: usize, col: usize, z_buffers: &Vec<ZBuffer>, background_color: Color) -> Pixel {
    let mut blended_pixel: Pixel = Pixel::new(
        background_color.r,
        background_color.g,
        background_color.b,
    );

    for z_buffer in z_buffers {
        let row: isize = row as isize - z_buffer.y;
        let col: isize = col as isize - z_buffer.x;

        if row < 0 || row >= z_buffer.buffer.len() as isize ||
            col < 0 || col >= z_buffer.buffer[0].len() as isize {
            continue;
        }

        let depth_pixel = z_buffer.buffer
            .get(row as usize)
            .map(|row_pixels| row_pixels.get(col as usize));

        if let Some(Some(depth_pixel)) = depth_pixel {
            if depth_pixel.color.a != 0.0 {
                let mut depth_pixel_color: Color = depth_pixel.color.clone();

                let alpha: f32 = depth_pixel_color.a;

                blended_pixel = Pixel::new(
                    ((1.0 - alpha) * blended_pixel.r as f32 + alpha * depth_pixel_color.r as f32) as u8,
                    ((1.0 - alpha) * blended_pixel.g as f32 + alpha * depth_pixel_color.g as f32) as u8,
                    ((1.0 - alpha) * blended_pixel.b as f32 + alpha * depth_pixel_color.b as f32) as u8,
                );

                // let pixel: Pixel = Pixel::new(
                //     ((1.0 - alpha) * background_color.r as f32 + alpha * depth_pixel_color.r as f32) as u8,
                //     ((1.0 - alpha) * background_color.g as f32 + alpha * depth_pixel_color.g as f32) as u8,
                //     ((1.0 - alpha) * background_color.b as f32 + alpha * depth_pixel_color.b as f32) as u8,
                // );
                //
                // return pixel;
            }
        }
    }

    blended_pixel
}
