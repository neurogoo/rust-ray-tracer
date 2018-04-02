extern crate rand;
extern crate rayon;
extern crate stb_image;

use std::fs::File;
use std::io::Write;
use std::f32;
use std::time::{Duration, Instant};
use rand::{thread_rng, Rng};
use rayon::prelude::*;

mod vector;
mod ray;
mod hitable;
mod sphere;
mod camera;
mod material;
mod utils;
mod texture;
mod perlin;

use vector::*;
use ray::*;
use hitable::*;
use camera::*;
use material::*;
use utils::*;

fn color(r: &Ray, world: &Hitable, depth: u32) -> Vec3 {
    match world.hit(r, 0.001, f32::MAX) {
        Some(rec) => {
            let emitted = rec.material.emitted(rec.u, rec.v, &rec.p);
            if depth < 50 {
                match rec.material.scatter(r, &rec) {
                    Scattered::Yes((scattered, attenuation)) => {
                        return emitted + attenuation * color(&scattered, world, depth + 1)
                    }
                    _ => return emitted,
                }
            }
            emitted
        }
        None => Vec3(0.0, 0.0, 0.0),
    }
}

fn main() {
    let now = Instant::now();
    const NX: u32 = 600;
    const NY: u32 = 300;
    let ns = 500;
    println!("Making picture");
    let mut f = File::create("/home/tokuogum/Rust/rust-ray-tracer/picture.ppm")
        .expect("Couldn't create picture file");
    write!(&mut f, "P3\n{} {}\n255\n", NX, NY).unwrap();

    // let mut world_list = random_scene();
    // let lookfrom = Vec3(13.0, 2.0, 3.0);
    // let lookat = Vec3(0.0, 0.0, 0.0);
    // //   let dist_to_focus = (lookfrom - lookat).length();
    // let dist_to_focus = 10.0;
    // let aperture = 0.0;
    // let camera = Camera::new(
    //     lookfrom,
    //     lookat,
    //     Vec3(0.0, 1.0, 0.0),
    //     20.0,
    //     nx as f32 / ny as f32,
    //     aperture,
    //     dist_to_focus,
    //     0.0,
    //     1.0,
    // );

    // let mut world_list = simple_light();
    // //    let lookfrom = Vec3(13.0, 2.0, 3.0);
    // let lookfrom = Vec3(25.0, 2.0, 3.0);
    // let lookat = Vec3(0.0, 0.0, 0.0);
    // let dist_to_focus = 10.0;
    // let aperture = 0.0;
    // let camera = Camera::new(
    //     lookfrom,
    //     lookat,
    //     Vec3(0.0, 1.0, 0.0),
    //     20.0,
    //     NX as f32 / NY as f32,
    //     aperture,
    //     dist_to_focus,
    //     0.0,
    //     1.0,
    // );

    let (camera, mut world_list) = cornell_box(NX, NY);

    println!("Starting making bvh at {}", now.elapsed().as_secs());
    let bbox = Hitable::BvhNode(BvhNode::new(&mut world_list, 0.0, 1.0));
    println!("Finishing making bvh at {}", now.elapsed().as_secs());
    let mut picture_array: Vec<Vec<(u32, u32, u32)>> = Vec::new();
    for j in (0..NY).rev() {
        picture_array.push(
            (0..NX)
                .into_par_iter()
                .map(|i| {
                    let mut col: Vec3 = (0..ns)
                        .into_par_iter()
                        .map(|_| {
                            let mut rng = thread_rng();
                            let u = (i as f32 + rng.gen::<f32>()) / NX as f32;
                            let v = (j as f32 + rng.gen::<f32>()) / NY as f32;
                            let ray = camera.get_ray(u, v);
                            color(&ray, &bbox, 0)
                        })
                        .reduce_with(|sum, val| sum + val)
                        .unwrap();
                    col = col / (ns as f32);
                    col = Vec3(col.0.sqrt(), col.1.sqrt(), col.2.sqrt());
                    let ir = (255.99 * col.r()) as u32;
                    let ig = (255.99 * col.g()) as u32;
                    let ib = (255.99 * col.b()) as u32;
                    (ir, ig, ib)
                })
                .collect(),
        );
    }
    for j in (0..NY) {
        for i in 0..NX {
            let (ir, ig, ib) = picture_array[j as usize][i as usize];
            write!(&mut f, "{} {} {}\n", ir, ig, ib).unwrap();
        }
    }
    println!("Elapsed time {}", now.elapsed().as_secs());
}
