use crate::renderer::scene::object_3d::point::Point;

pub type TriangleVertices = [Point; 3];

pub struct Triangle {
    pub vertices: TriangleVertices
}

impl Triangle {
    pub fn new(vertices: TriangleVertices) -> Triangle {
        Triangle {
            vertices
        }
    }
}
