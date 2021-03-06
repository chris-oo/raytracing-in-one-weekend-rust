use crate::utility;
use crate::utility::{random_f64, random_f64_range};
use std::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub};

/// Simple vec3 class
/// Laid out as x, y, z
#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct Vec3(f64, f64, f64);

#[allow(dead_code)]
impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3(x, y, z)
    }

    pub fn x(&self) -> f64 {
        self.0
    }

    pub fn y(&self) -> f64 {
        self.1
    }

    pub fn z(&self) -> f64 {
        self.2
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
        u.0 * v.0 + u.1 * v.1 + u.2 * v.2
    }

    pub fn cross(u: &Vec3, v: &Vec3) -> Self {
        Vec3(
            u.1 * v.2 - u.2 * v.1,
            u.2 * v.0 - u.0 * v.2,
            u.0 * v.1 - u.1 * v.0,
        )
    }

    pub fn unit_vector(v: Vec3) -> Self {
        v / v.length()
    }

    /// Get a random unit vector.
    pub fn random() -> Self {
        Vec3(random_f64(), random_f64(), random_f64())
    }

    /// Get a random vector with a given min/max range.
    pub fn random_range(min: f64, max: f64) -> Self {
        Vec3(
            random_f64_range(min, max),
            random_f64_range(min, max),
            random_f64_range(min, max),
        )
    }

    /// Get a random vector within a unit sphere.
    pub fn random_in_unit_sphere() -> Self {
        loop {
            let p = Vec3::random_range(-1.0, 1.0);
            if p.length_squared() >= 1.0 {
                continue;
            }
            return p;
        }
    }

    /// Get a Lambertian distrubuted unit vector, see Section 8.5.
    pub fn random_unit_vector() -> Self {
        let a = random_f64_range(0.0, 2.0 * utility::PI);
        let z = random_f64_range(-1.0, 1.0);
        let r = f64::sqrt(1.0 - z * z);
        Vec3(r * a.cos(), r * a.sin(), z)
    }

    /// Get an alternative diffuse vector, see Section 8.6.
    pub fn random_in_hemisphere(normal: &Vec3) -> Self {
        let in_unit_sphere = Vec3::random_in_unit_sphere();
        if Vec3::dot(&in_unit_sphere, normal) > 0.0
        // In the same hemisphere as the normal
        {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }

    pub fn reflect(v: &Vec3, n: &Vec3) -> Self {
        *v - 2.0 * Vec3::dot(v, n) * (*n)
    }

    pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = Vec3::dot(&-*uv, n);
        let r_out_parallel = etai_over_etat * (*uv + cos_theta * (*n));
        let r_out_perp = -f64::sqrt(1.0 - r_out_parallel.length_squared()) * (*n);
        r_out_parallel + r_out_perp
    }

    pub fn random_in_unit_disk() -> Self {
        loop {
            let p = Vec3(
                random_f64_range(-1.0, 1.0),
                random_f64_range(-1.0, 1.0),
                0.0,
            );
            if p.length_squared() >= 1.0 {
                continue;
            }
            return p;
        }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3(-self.0, -self.1, -self.2)
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            _ => panic!("Vec3 index out of bounds"),
        }
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            _ => panic!("Vec3 index out of bounds"),
        }
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vec3(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
        self.1 += other.1;
        self.2 += other.2;
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Vec3(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Vec3(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3(self * rhs.0, self * rhs.1, self * rhs.2)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        rhs * self
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.0 *= rhs;
        self.1 *= rhs;
        self.2 *= rhs;
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        (1.0 / rhs) * self
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0 / rhs;
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.0, self.1, self.2)
    }
}

// Point3 and Vec3 are just aliases for each other
// TODO - Probably should use newtype here? but it results in all annoying
// non-automatic from conversion in operators, even with deriving
pub type Point3 = Vec3;

// Use Newtype pattern for Color

macro_attr! {
    /// RGB Color (r, g, b)
    #[derive(Clone, Copy, Debug,
        NewtypeAdd!, NewtypeAddAssign!, NewtypeSub!,
        NewtypeMul!, NewtypeMulAssign!(f64),
        NewtypeDiv!(f64), NewtypeDivAssign!(f64),
        NewtypeDeref!, NewtypeDerefMut!)]
    pub struct Color(Vec3);
}

// TODO - Any way to derive this?
impl Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Color(Vec3(self * rhs.x(), self * rhs.y(), self * rhs.z()))
    }
}

// impl fmt::Display for Color {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         // Write the translated [0,255] value of each color component.
//         write!(
//             f,
//             "{} {} {}",
//             (255.999 * self.x()) as i32,
//             (255.999 * self.y()) as i32,
//             (255.999 * self.z()) as i32,
//         )
//     }
// }

#[allow(dead_code)]
impl Color {
    // Create a new color with rgb values.
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Color(Vec3(r, g, b))
    }

    /// Explicit conversion from Vec3 to color.
    pub fn from_vec(vec: Vec3) -> Self {
        Color(vec)
    }

    pub fn random() -> Self {
        Color(Vec3(random_f64(), random_f64(), random_f64()))
    }

    pub fn get_color_string(&self, samples_per_pixel: i32) -> String {
        let mut r: f64 = self.x();
        let mut g: f64 = self.y();
        let mut b: f64 = self.z();

        // Divide the color total by the number of samples and gamma-correct for gamma=2.0.
        let scale = 1.0 / samples_per_pixel as f64;
        r = f64::sqrt(scale * r);
        g = f64::sqrt(scale * g);
        b = f64::sqrt(scale * b);

        // Write the translated [0,255] value of each color component.
        format!(
            "{} {} {}\n",
            (256.0 * utility::clamp(r, 0.0, 0.999)) as i32,
            (256.0 * utility::clamp(g, 0.0, 0.999)) as i32,
            (256.0 * utility::clamp(b, 0.0, 0.999)) as i32,
        )
    }
}

// TODO - more tests
#[cfg(test)]
mod tests {
    use crate::vec3::Vec3;

    #[test]
    fn vec3_binary_ops() {
        assert_eq!(
            Vec3(4.0, 3.0, 2.0) - Vec3(2.0, 1.0, 0.5),
            Vec3(2.0, 2.0, 1.5)
        );
    }
}
