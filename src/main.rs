extern crate rand;

use std::fs::File;
use std::io::Write;
use std::f32;
use rand::{thread_rng, Rng};

mod vector;
mod ray;
mod hitable;
mod sphere;
mod camera;
mod material;
mod utils;

use vector::*;
use ray::*;
use hitable::*;
use sphere::*;
use camera::*;
use material::*;
use utils::*;

fn color(r: &Ray, world: &HitableList, depth: u32) -> Vec3 {
    match world.hit(r, 0.001, f32::MAX) {
        Some(rec) => {
            if depth < 50 {
                match rec.material.scatter(r, &rec) {
                    Some((scattered, attenuation)) => {
                        return attenuation * color(&scattered, world, depth + 1)
                    }
                    None => {}
                }
            }
            Vec3(0.0, 0.0, 0.0)
        }
        None => {
            let unit_direction = unit_vector(r.direction());
            let t = 0.5 * (unit_direction.y() + 1.0);
            return Vec3(1.0, 1.0, 1.0) * (1.0 - t) + Vec3(0.5, 0.7, 1.0) * t;
        }
    }
}

fn main() {
    let nx = 200;
    let ny = 100;
    let ns = 100;
    println!("Making picture");
    let mut f = File::create("/home/tokuogum/Rust/rust-ray-tracer/picture.ppm")
        .expect("Couldn't create picture file");
    write!(&mut f, "P3\n{} {}\n255\n", nx, ny).unwrap();
    let lam1 = Labertian::new(Vec3(0.8, 0.3, 0.3));
    let lam2 = Labertian::new(Vec3(0.8, 0.8, 0.0));
    let met1 = Metal::new(Vec3(0.8, 0.6, 0.2), 1.0);
    let met2 = Metal::new(Vec3(0.8, 0.8, 0.8), 0.3);
    let mut world = HitableList::new();
    world
        .list
        .push(Box::new(Sphere::new(Vec3(0.0, 0.0, -1.0), 0.5, &lam1)));
    world
        .list
        .push(Box::new(Sphere::new(Vec3(0.0, -100.5, -1.0), 100.0, &lam2)));
    world
        .list
        .push(Box::new(Sphere::new(Vec3(1.0, 0.0, -1.0), 0.5, &met1)));
    world
        .list
        .push(Box::new(Sphere::new(Vec3(-1.0, 0.0, -1.0), 0.5, &met2)));
    let camera = Camera::new();
    let mut rng = thread_rng();
    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3(0.0, 0.0, 0.0);
            for _s in 0..ns {
                let u = (i as f32 + rng.gen::<f32>()) / nx as f32;
                let v = (j as f32 + rng.gen::<f32>()) / ny as f32;
                let ray = camera.get_ray(u, v);
                let _p = ray.point_at_parameter(2.0);
                col = col + color(&ray, &world, 0);
            }
            col = col / (ns as f32);
            col = Vec3(col.0.sqrt(), col.1.sqrt(), col.2.sqrt());
            let ir = (255.99 * col.r()) as u32;
            let ig = (255.99 * col.g()) as u32;
            let ib = (255.99 * col.b()) as u32;
            write!(&mut f, "{} {} {}\n", ir, ig, ib).unwrap();
        }
    }
}
