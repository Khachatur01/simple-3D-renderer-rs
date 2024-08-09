use crate::rendering_engine::engine::pixel::Pixel;
use crate::rendering_engine::engine::rasterizer::ZBuffer;
use crate::rendering_engine::scene::camera::display::Display;

pub type Image = Vec<Vec<Pixel>>;

/* @FIXME refactor */

pub fn composite(z_buffers: &Vec<ZBuffer>, display: &Display) -> Image {
    let mut image = vec![
        vec![Pixel::default(); display.width];
        display.height
    ];

    for (row, row_pixels) in image.iter_mut().enumerate() {
        for (col, pixel) in row_pixels.iter_mut().enumerate() {
            let result_pixel = composite_pixel(row, col, &z_buffers);

            pixel.color = result_pixel.color.clone();
        }
    }

    image
}

fn composite_pixel(row: usize, col: usize, z_buffers: &Vec<ZBuffer>) -> Pixel {
    let mut pixel: Pixel = Pixel::default();

    for z_buffer in z_buffers {
        let a = z_buffer.buffer
            .get(row + z_buffer.y)
            .map(|row_pixels| row_pixels.get(col + z_buffer.x));

        if let Some(Some(depth_pixel)) = a {
            if depth_pixel.pixel.color.a != 0 {
                return depth_pixel.pixel.clone();
            }
        }
    }

    pixel
}