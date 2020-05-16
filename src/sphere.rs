use crate::hit::HitRecord;
use crate::hit::Hittable;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use std::rc::Rc;

pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Rc<dyn Material>) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }

    pub fn new_rc(center: Point3, radius: f64, material: Rc<dyn Material>) -> Rc<Self> {
        Rc::new(Sphere::new(center, radius, material))
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = Vec3::dot(&oc, &r.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let temp = (-half_b - root) / a;
            if temp < t_max && temp > t_min {
                let t = temp;
                let p = r.at(t);
                let outward_normal = (p - self.center) / self.radius;
                return Some(HitRecord::new(
                    p,
                    r,
                    outward_normal,
                    t,
                    self.material.clone(),
                ));
            }

            let temp = (-half_b + root) / a;
            if temp < t_max && temp > t_min {
                let t = temp;
                let p = r.at(t);
                let outward_normal = (p - self.center) / self.radius;
                return Some(HitRecord::new(
                    p,
                    r,
                    outward_normal,
                    t,
                    self.material.clone(),
                ));
            }
        }

        None
    }
}
