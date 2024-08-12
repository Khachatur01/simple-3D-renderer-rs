use crate::rendering_engine::scene::model::color::Color;

#[derive(Debug, Clone, Copy)]
pub struct ZBufferedVertex {
    pub x: f32,
    pub y: f32,
    pub distance: f32,
}

pub type ZBufferedVertices = [ZBufferedVertex; 3];

pub struct ZBufferedTriangle {
    pub vertices: ZBufferedVertices,
    pub color: Color,
}
