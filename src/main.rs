use crate::hit::{Hittable, HittableList};
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::{Color, Point3, Vec3};
use std::io::{self, Write};

#[macro_use]
extern crate macro_attr;
#[macro_use]
extern crate newtype_derive;

mod hit;
mod ray;
mod sphere;
mod utility;
mod vec3;

fn ray_color(r: &Ray, world: &dyn Hittable) -> Color {
    let hit = world.hit(r, 0.0, f64::INFINITY);

    if let Some(rec) = hit {
        return Color::from_vec(0.5 * (rec.normal + *Color::new(1.0, 1.0, 1.0)));
    }

    let unit_direction = Vec3::unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width: i32 = 384;
    let image_height: i32 = (image_width as f64 / aspect_ratio) as i32;

    print!("P3\n{} {} \n255\n", image_width, image_height);

    let lower_left_corner = Point3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.25, 0.0);
    let origin = Point3::new(0.0, 0.0, 0.0);

    let mut world = HittableList::default();
    world.add(Sphere::new_rc(Point3::new(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new_rc(Point3::new(0.0, -100.5, -1.0), 100.0));

    (0..image_height).rev().for_each(|j| {
        eprint!("\rScanlines remaining: {:4}", j);
        io::stderr().flush().unwrap();

        (0..image_width).for_each(|i| {
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;
            let r = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical);

            let pixel_color = ray_color(&r, &world);

            print!("{}\n", pixel_color);
        });
    });

    eprint!("\nDone.\n");
}
