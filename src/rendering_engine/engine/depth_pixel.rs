use crate::rendering_engine::engine::pixel::Pixel;

pub struct DepthPixel {
    pixel: Pixel,
    depth: f32, /* value should be from 0 to 1 */
}