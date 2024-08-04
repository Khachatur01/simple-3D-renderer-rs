pub mod coefficients;

use crate::renderer::scene::object_3d::plane::coefficients::Coefficients;
use crate::renderer::scene::object_3d::point::Point;
use crate::renderer::scene::object_3d::vector::Vector;

pub struct Plane {
    /** normal vector of plane */
    pub normal: Vector,
    /** a point in plane */
    pub point: Point,
    /** coefficients of plane equation */
    pub coefficients: Coefficients
}

impl Plane {
    pub fn new(normal: Vector, point: Point) -> Plane {
        let Vector { x: a, y: b, z: c } = normal;

        let d: f32 = a*normal.x + b*normal.y + c*normal.z;

        Plane {
            normal,
            point,
            coefficients: Coefficients { a, b, c, d, }
        }
    }
}