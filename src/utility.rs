/// pi in the book is defined different than the std f64 one.
pub const PI: f64 = 3.1415926535897932385;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

// Take clamp from nightly
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
