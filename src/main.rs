use crate::camera::Camera;
use crate::hit::{Hittable, HittableList};
use crate::material::{Dielectric, Lambertian, Metal};
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::utility::random_f64;
use crate::vec3::{Color, Point3, Vec3};
use rayon::prelude::*;
use std::io::{self, Write};
use std::sync::Arc;
use std::sync::Mutex;

#[macro_use]
extern crate macro_attr;
#[macro_use]
extern crate newtype_derive;

mod camera;
mod hit;
mod material;
mod ray;
mod sphere;
mod utility;
mod vec3;

fn decrement_and_print_lines_remaining(count: Arc<Mutex<i32>>) {
    let mut val = count.lock().unwrap();
    *val -= 1;

    if *val % 25 == 0 {
        eprint!("\rRender Scanlines remaining: {:4}", val);
        io::stderr().flush().unwrap();
    }
}

fn ray_color(r: &Ray, world: &dyn Hittable, depth: i32) -> Color {
    // If we've exceeded the ray bounce limit, no more light is gathered.
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if let Some(rec) = world.hit(r, 0.001, f64::INFINITY) {
        if let Some((attenuation, scattered)) = rec.material.scatter(r, &rec) {
            return attenuation * ray_color(&scattered, world, depth - 1);
        } else {
            return Color::new(0.0, 0.0, 0.0);
        }
    }

    let unit_direction = Vec3::unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width: i32 = 384;
    let image_height: i32 = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    print!("P3\n{} {} \n255\n", image_width, image_height);

    let mut world = HittableList::default();
    world.add(Sphere::new_arc(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        Lambertian::new(Color::new(0.1, 0.2, 0.5)),
    ));
    world.add(Sphere::new_arc(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        Lambertian::new(Color::new(0.8, 0.8, 0.0)),
    ));

    world.add(Sphere::new_arc(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        Metal::new(Color::new(0.8, 0.6, 0.2), 0.0),
    ));
    world.add(Sphere::new_arc(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        Dielectric::new(1.5),
    ));
    world.add(Sphere::new_arc(
        Point3::new(-1.0, 0.0, -1.0),
        -0.45,
        Dielectric::new(1.5),
    ));

    let camera = Camera::new(
        Point3::new(-2.0, 2.0, 1.0),
        Point3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        aspect_ratio,
    );

    let line_count_remaining = Arc::new(Mutex::new(image_height));

    let image: Vec<Vec<_>> = (0..image_height)
        .into_par_iter()
        .rev()
        .map(|j| {
            let scanline: Vec<_> = (0..image_width)
                .map(|i| {
                    let mut pixel_color = Color::new(0.0, 0.0, 0.0);

                    (0..samples_per_pixel).for_each(|_| {
                        let u = (i as f64 + random_f64()) / (image_width - 1) as f64;
                        let v = (j as f64 + random_f64()) / (image_height - 1) as f64;
                        let r = camera.get_ray(u, v);
                        pixel_color += ray_color(&r, &world, max_depth);
                    });

                    pixel_color
                })
                .collect();

            decrement_and_print_lines_remaining(line_count_remaining.clone());

            scanline
        })
        .collect();

    eprint!("\nPrinting...");

    image.iter().for_each(|v| {
        v.iter().for_each(|color| {
            print!("{}", color.get_color_string(samples_per_pixel));
        })
    });

    eprint!("\nDone.\n");
}
