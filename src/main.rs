use crate::camera::Camera;
use crate::hit::{Hittable, HittableList};
use crate::material::Material;
use crate::material::{Dielectric, Lambertian, Metal};
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::utility::random_f64;
use crate::utility::random_f64_range;
use crate::vec3::{Color, Point3, Vec3};
use rayon::prelude::*;
use std::io::{self, Write};
use std::sync::Arc;

use std::sync::mpsc::channel;
use std::thread;

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

fn random_scene() -> HittableList {
    let mut world = HittableList::default();

    let ground_material = Lambertian::new(Color::new(0.5, 0.5, 0.5));
    world.add(Sphere::new_arc(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    ));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_f64();
            let center = Point3::new(
                a as f64 + 0.9 * random_f64(),
                0.2,
                b as f64 + 0.9 * random_f64(),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Arc<dyn Material + Send + Sync> = if choose_mat < 0.8 {
                    // Diffuse
                    let albedo = Color::random() * Color::random();
                    Lambertian::new(albedo)
                } else if choose_mat < 0.95 {
                    // Metal
                    let albedo = Color::random();
                    let fuzz = random_f64_range(0.0, 0.5);
                    Metal::new(albedo, fuzz)
                } else {
                    // Glass
                    Dielectric::new(1.5)
                };

                world.add(Sphere::new_arc(center, 0.2, sphere_material));
            }
        }
    }

    let material1 = Dielectric::new(1.5);
    world.add(Sphere::new_arc(Point3::new(0.0, 1.0, 0.0), 1.0, material1));

    let material2 = Lambertian::new(Color::new(0.4, 0.2, 0.1));
    world.add(Sphere::new_arc(Point3::new(-4.0, 1.0, 0.0), 1.0, material2));

    let material3 = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);
    world.add(Sphere::new_arc(Point3::new(4.0, 1.0, 0.0), 1.0, material3));

    world
}

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width: i32 = 1200;
    let image_height: i32 = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    print!("P3\n{} {} \n255\n", image_width, image_height);

    let world = random_scene();

    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

    let (send, recv) = channel::<i32>();

    thread::spawn(move || {
        let mut scanlines_remaining = image_height;

        while scanlines_remaining > 0 {
            recv.recv().unwrap();
            scanlines_remaining -= 1;

            eprint!("\rRender Scanlines remaining: {:4}", scanlines_remaining);
            io::stderr().flush().unwrap();
        }
    });

    let image: Vec<Vec<_>> = (0..image_height)
        .into_par_iter()
        .rev()
        .map_with(send, |s, j| {
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

            s.send(1).unwrap();

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
