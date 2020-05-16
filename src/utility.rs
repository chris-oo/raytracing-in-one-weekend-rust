/// pi in the book is defined different than the std f64 one.
pub const PI: f64 = 3.1415926535897932385;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    assert!(min <= max);
    let mut x = x;

    if x < min {
        x = min;
    }
    if x > max {
        x = max;
    }
    x
}

pub fn random_f64() -> f64 {
    rand::random::<f64>()
}

// Random value for a range [min, max)
pub fn random_f64_range(min: f64, max: f64) -> f64 {
    min + (max - min) * rand::random::<f64>()
}
