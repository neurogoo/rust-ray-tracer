use std::fs::File;
use std::io::Write;

mod vector;
mod ray;

use vector::*;
use ray::*;

fn color(r: &Ray) -> Vec3 {
    if hit_sphere(&Vec3(0.0, 0.0, -1.0), 0.5, r) {
        return Vec3(1.0, 0.0, 0.0);
    }
    let unit_direction = unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    Vec3(1.0, 1.0, 1.0) * (1.0 - t) + Vec3(0.5, 0.7, 1.0) * t
}

fn hit_sphere(center: &Vec3, radius: f32, ray: &Ray) -> bool {
    let oc = ray.origin() - *center;
    let a = dot(&ray.direction(), &ray.direction());
    let b = dot(&oc, &ray.direction()) * 2.0;
    let c = dot(&oc, &oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    discriminant > 0.0
}

fn main() {
    let nx = 200;
    let ny = 100;
    println!("Making picture");
    let mut f = File::create("/home/tokuogum/Rust/rust-ray-tracer/picture.ppm")
        .expect("Couldn't create picture file");
    write!(&mut f, "P3\n{} {}\n255\n", nx, ny).unwrap();

    let lower_left_corner = Vec3(-2.0, -1.0, -1.0);
    let horizontal = Vec3(4.0, 0.0, 0.0);
    let vertical = Vec3(0.0, 2.0, 0.0);
    let origin = Vec3(0.0, 0.0, 0.0);

    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = i as f32 / nx as f32;
            let v = j as f32 / ny as f32;
            let ray = Ray::new(origin, lower_left_corner + horizontal * u + vertical * v);
            let col = color(&ray);
            let ir = (255.99 * col.r()) as u32;
            let ig = (255.99 * col.g()) as u32;
            let ib = (255.99 * col.b()) as u32;
            write!(&mut f, "{} {} {}\n", ir, ig, ib).unwrap();
        }
    }
}
