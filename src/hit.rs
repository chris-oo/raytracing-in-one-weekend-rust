use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use std::rc::Rc;

#[derive(Debug, Clone, Copy)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    // pub fn new(p: Point3, normal: Vec3, t: f64) -> Self {
    //     HitRecord {
    //         p,
    //         normal,
    //         t,
    //         front_face: false,
    //     }
    // }

    /// Construct a new hit record using the specified point, time, and
    /// ray and outward normal to calculate the normal and if this hit record
    /// is facing the front or not.
    pub fn new(p: Point3, r: &Ray, outward_normal: Vec3, t: f64) -> Self {
        let front_face = Vec3::dot(&r.direction(), &outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        HitRecord {
            p,
            normal,
            t,
            front_face,
        }
    }

    // pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
    //     self.front_face = Vec3::dot(&r.direction(), outward_normal) < 0.0;
    //     self.normal = if self.front_face {
    //         *outward_normal
    //     } else {
    //         -*outward_normal
    //     };
    // }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

impl std::fmt::Debug for dyn Hittable {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Default)]
pub struct HittableList {
    objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit = None;
        let mut closest_so_far = t_max;

        self.objects.iter().for_each(|object| {
            if let Some(new_hit) = object.hit(r, t_min, closest_so_far) {
                hit = Some(new_hit);
                closest_so_far = new_hit.t;
            }
        });

        hit
    }
}
