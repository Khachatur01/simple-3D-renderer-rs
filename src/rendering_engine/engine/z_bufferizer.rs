use crate::rendering_engine::engine::depth_pixel::DepthPixel;
use crate::rendering_engine::scene::model_2d::triangle::Triangle as Triangle2D;
use crate::rendering_engine::scene::model_3d::triangle::Triangle as Triangle3D;

pub type ZBuffer = Vec<Vec<DepthPixel>>;

pub fn z_buffer(triangle2d: &Triangle2D, triangle3d: &Triangle3D) -> ZBuffer {
    vec![]
}
