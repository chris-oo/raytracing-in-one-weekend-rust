use crate::vec3::{Point3, Vec3};

#[derive(Debug, Default)]
pub struct Ray {
    origin: Point3,
    direction: Vec3,
}

#[allow(dead_code)]
impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Ray { origin, direction }
    }

    pub fn origin(&self) -> Point3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + Point3::from(t * self.direction)
    }
}
