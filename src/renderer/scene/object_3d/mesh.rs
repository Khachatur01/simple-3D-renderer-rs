use crate::renderer::scene::object_2d::point::Point as Point2D;
use crate::renderer::scene::object_3d::point::Point as Point3D;
use crate::renderer::scene::object_3d::triangle::Triangle as Triangle3D;

pub struct Mesh {
    vertices: Vec<Point3D>
}

impl Mesh {
    pub fn new(vertices: Vec<Point3D>) -> Mesh {
        Mesh {
            vertices
        }
    }

    pub fn triangulate(&self) -> Vec<Triangle3D> {
        /* @FIXME */
        vec![
            Triangle3D {
                vertices: [
                    Point3D { x: -150.0, y: -50.0, z: 250.0 },
                    Point3D { x: -150.0, y: 50.0, z: 250.0 },
                    Point3D { x: -50.0, y: 50.0, z: 250.0 },
                ]
            }
        ]
    }
}