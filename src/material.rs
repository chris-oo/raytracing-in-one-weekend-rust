use crate::hit::HitRecord;
use crate::ray::Ray;
use crate::utility::random_f64;
use crate::vec3::Color;
use crate::vec3::Vec3;
use std::sync::Arc;

pub trait Material {
    /// Returns the attenuation and scatter ray by the material in the
    /// form of Option<(Color, Ray)> if the material did not absorb the ray.
    ///
    /// A material that absorbs the ray returns None.
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
}

impl std::fmt::Debug for dyn Material {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Arc<Self> {
        Arc::new(Lambertian { albedo })
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let scatter_direction = rec.normal + Vec3::random_unit_vector();
        let scattered = Ray::new(rec.p, scatter_direction);
        let attenuation = self.albedo;
        Some((attenuation, scattered))
    }
}

#[derive(Debug)]
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Arc<Self> {
        Arc::new(Metal {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        })
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = Vec3::reflect(&Vec3::unit_vector(r_in.direction()), &rec.normal);
        let scattered = Ray::new(rec.p, reflected + self.fuzz * Vec3::random_in_unit_sphere());
        let attenuation = self.albedo;
        if Vec3::dot(&scattered.direction(), &rec.normal) > 0.0 {
            Some((attenuation, scattered))
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct Dielectric {
    ref_idx: f64,
}

impl Dielectric {
    pub fn new(ref_idx: f64) -> Arc<Self> {
        Arc::new(Dielectric { ref_idx })
    }

    fn schlick(cosine: f64, ref_idx: f64) -> f64 {
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * f64::powi(1.0 - cosine, 5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let etai_over_etat = if rec.front_face {
            1.0 / self.ref_idx
        } else {
            self.ref_idx
        };

        let unit_direction = Vec3::unit_vector(r_in.direction());
        let cos_theta = f64::min(Vec3::dot(&-unit_direction, &rec.normal), 1.0);
        let sin_theta = f64::sqrt(1.0 - cos_theta * cos_theta);

        if etai_over_etat * sin_theta > 1.0 {
            let reflected = Vec3::reflect(&unit_direction, &rec.normal);
            let scattered = Ray::new(rec.p, reflected);
            return Some((attenuation, scattered));
        }

        let reflect_prob = Dielectric::schlick(cos_theta, etai_over_etat);
        if random_f64() < reflect_prob {
            let reflected = Vec3::reflect(&unit_direction, &rec.normal);
            let scattered = Ray::new(rec.p, reflected);
            return Some((attenuation, scattered));
        }

        let refracted = Vec3::refract(&unit_direction, &rec.normal, etai_over_etat);
        let scattered = Ray::new(rec.p, refracted);
        Some((attenuation, scattered))
    }
}
