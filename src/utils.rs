use std::f32;
use rand::{thread_rng, Rng};

use vector::*;
use hitable::*;
use material::*;
use sphere::*;

pub fn random_in_unit_sphere() -> Vec3 {
    let mut rng = thread_rng();
    let mut p;
    loop {
        p = Vec3(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>()) * 2.0 - Vec3(1.0, 1.0, 1.0);
        if p.squared_length() < 1.0 {
            break;
        }
    }
    p
}

pub fn random_scene<'a>() -> HitableList<'a> {
    let mut rng = thread_rng();
    let n = 50000;
    let mut world = HitableList::new();
    world.list.push(Box::new(Sphere::new(
        Vec3(0.0, -1000.0, 0.0),
        1000.0,
        Box::new(Labertian::new(Vec3(0.5, 0.5, 0.5))),
    )));
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen::<f32>();
            let center = Vec3(
                a as f32 + 0.9 * rng.gen::<f32>(),
                0.2,
                b as f32 + 0.9 * rng.gen::<f32>(),
            );
            if (center - Vec3(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    world.list.push(Box::new(MovingSphere::new(
                        center,
                        center + Vec3(0.0, 0.5 * rng.gen::<f32>(), 0.0),
                        0.0,
                        1.0,
                        0.2,
                        Box::new(Labertian::new(Vec3(
                            rng.gen::<f32>() * rng.gen::<f32>(),
                            rng.gen::<f32>() * rng.gen::<f32>(),
                            rng.gen::<f32>() * rng.gen::<f32>(),
                        ))),
                    )))
                } else if choose_mat < 0.95 {
                    world.list.push(Box::new(Sphere::new(
                        center,
                        0.2,
                        Box::new(Metal::new(
                            Vec3(
                                0.5 * (1.0 + rng.gen::<f32>()),
                                0.5 * (1.0 + rng.gen::<f32>()),
                                0.5 * (1.0 + rng.gen::<f32>()),
                            ),
                            0.5 * rng.gen::<f32>(),
                        )),
                    )))
                } else {
                    world.list.push(Box::new(Sphere::new(
                        center,
                        0.2,
                        Box::new(Dielectric::new(1.5)),
                    )))
                }
            }
        }
    }
    world.list.push(Box::new(Sphere::new(
        Vec3(0.0, 1.0, 0.0),
        1.0,
        Box::new(Dielectric::new(1.5)),
    )));
    world.list.push(Box::new(Sphere::new(
        Vec3(-4.0, 1.0, 0.0),
        1.0,
        Box::new(Labertian::new(Vec3(0.4, 0.2, 0.1))),
    )));
    world.list.push(Box::new(Sphere::new(
        Vec3(4.0, 1.0, 0.0),
        1.0,
        Box::new(Metal::new(Vec3(0.7, 0.6, 0.5), 0.0)),
    )));
    world
}
