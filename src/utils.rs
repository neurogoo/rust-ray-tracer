use std::f32;
use rand::{thread_rng, Rng};

use vector::*;
use hitable::*;
use material::*;
use sphere::*;
use ray::*;
use texture::*;

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

fn new_sphere(cen: Vec3, r: f32, material: Material) -> Hitable {
    Hitable::Sphere(Sphere::new(cen, r, material))
}

fn new_labertian(albedo: Texture) -> Material {
    Material::Labertian(Labertian::new(albedo))
}

pub fn two_perlin_spheres() -> Vec<Hitable> {
    let pertext = Texture::NoiseTexture(NoiseTexture::new());
    let mut hitables: Vec<Hitable> = Vec::new();
    hitables.push(new_sphere(Vec3(0.0, -1000.0, 0.0), 1000.0, new_labertian(pertext.clone())));
    hitables.push(new_sphere(Vec3(0.0, 2.0, 0.0), 2.0, new_labertian(pertext.clone())));
    hitables
}

pub fn random_scene() -> Vec<Hitable> {
    let mut rng = thread_rng();
    let n = 50000;
    let mut world: Vec<Hitable> = Vec::new();
    let checker = Texture::CheckedTexture(CheckerTexture::new(
        Texture::ConstantTexture(ConstantTexture::new(Vec3(0.2, 0.3, 0.1))),
        Texture::ConstantTexture(ConstantTexture::new(Vec3(0.9, 0.9, 0.9))),
    ));
    world.push(Hitable::Sphere(Sphere::new(
        Vec3(0.0, -1000.0, 0.0),
        1000.0,
        Material::Labertian(Labertian::new(checker)),
    )));
    for a in -10..10 {
        for b in -10..10 {
            let choose_mat = rng.gen::<f32>();
            let center = Vec3(
                a as f32 + 0.9 * rng.gen::<f32>(),
                0.2,
                b as f32 + 0.9 * rng.gen::<f32>(),
            );
            if (center - Vec3(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    world.push(Hitable::MovingSphere(MovingSphere::new(
                        center,
                        center + Vec3(0.0, 0.5 * rng.gen::<f32>(), 0.0),
                        0.0,
                        1.0,
                        0.2,
                        Material::Labertian(Labertian::new(Texture::ConstantTexture(
                            ConstantTexture::new(Vec3(
                                rng.gen::<f32>() * rng.gen::<f32>(),
                                rng.gen::<f32>() * rng.gen::<f32>(),
                                rng.gen::<f32>() * rng.gen::<f32>(),
                            )),
                        ))),
                    )))
                } else if choose_mat < 0.95 {
                    world.push(Hitable::Sphere(Sphere::new(
                        center,
                        0.2,
                        Material::Metal(Metal::new(
                            Vec3(
                                0.5 * (1.0 + rng.gen::<f32>()),
                                0.5 * (1.0 + rng.gen::<f32>()),
                                0.5 * (1.0 + rng.gen::<f32>()),
                            ),
                            0.5 * rng.gen::<f32>(),
                        )),
                    )))
                } else {
                    world.push(Hitable::Sphere(Sphere::new(
                        center,
                        0.2,
                        Material::Dielectric(Dielectric::new(1.5)),
                    )))
                }
            }
        }
    }
    world.push(Hitable::Sphere(Sphere::new(
        Vec3(0.0, 1.0, 0.0),
        1.0,
        Material::Dielectric(Dielectric::new(1.5)),
    )));
    world.push(Hitable::Sphere(Sphere::new(
        Vec3(-4.0, 1.0, 0.0),
        1.0,
        Material::Labertian(Labertian::new(Texture::ConstantTexture(
            ConstantTexture::new((Vec3(0.4, 0.2, 0.1))),
        ))),
    )));
    world.push(Hitable::Sphere(Sphere::new(
        Vec3(4.0, 1.0, 0.0),
        1.0,
        Material::Metal(Metal::new(Vec3(0.7, 0.6, 0.5), 0.0)),
    )));
    world
}

#[inline]
pub fn ffmin(a: f32, b: f32) -> f32 {
    if a < b {
        a
    } else {
        b
    }
}

#[inline]
pub fn ffmax(a: f32, b: f32) -> f32 {
    if a > b {
        a
    } else {
        b
    }
}

#[derive(Debug, Clone)]
pub struct Aabb {
    _min: Vec3,
    _max: Vec3,
}

impl Aabb {
    pub fn new(a: Vec3, b: Vec3) -> Aabb {
        Aabb { _min: a, _max: b }
    }

    pub fn min(&self) -> Vec3 {
        self._min
    }

    pub fn max(&self) -> Vec3 {
        self._max
    }

    pub fn hit(&self, r: &Ray, tmin: f32, tmax: f32) -> bool {
        for a in 0..3 {
            let t0 = ffmin(
                (self._min[a] - r.origin()[a]) / r.direction()[a],
                (self._max[a] - r.origin()[a]) / r.direction()[a],
            );
            let t1 = ffmax(
                (self._min[a] - r.origin()[a]) / r.direction()[a],
                (self._max[a] - r.origin()[a]) / r.direction()[a],
            );
            let tmin = ffmax(t0, tmin);
            let tmax = ffmin(t1, tmax);
            if tmax <= tmin {
                return false;
            }
        }
        true
    }
}

pub fn surrounding_box(box0: Aabb, box1: Aabb) -> Aabb {
    Aabb::new(
        Vec3(
            ffmin(box0.min().x(), box1.min().x()),
            ffmin(box0.min().y(), box1.min().y()),
            ffmin(box0.min().z(), box1.min().z()),
        ),
        Vec3(
            ffmax(box0.max().x(), box1.max().x()),
            ffmax(box0.max().y(), box1.max().y()),
            ffmax(box0.max().z(), box1.max().z()),
        ),
    )
}

    /*let mut world = HitableList::new();
    let r = (f32::consts::PI / 4.0).cos();
    world
        .list
        .push(Box::new(Sphere::new(Vec3(-r, 0.0, -1.0), r, &lam1test)));
    world
        .list
        .push(Box::new(Sphere::new(Vec3(r, 0.0, -1.0), r, &lam2test)));
    world.list.push(Box::new(Sphere::new(
        Vec3(0.0, 0.0, -1.0),
        0.5,
        Box::new(Labertian::new(Vec3(0.1, 0.2, 0.5))),
    )));
    world.list.push(Box::new(Sphere::new(
        Vec3(0.0, -100.5, -1.0),
        100.0,
        Box::new(Labertian::new(Vec3(0.8, 0.8, 0.0))),
    )));
    world.list.push(Box::new(Sphere::new(
        Vec3(1.0, 0.0, -1.0),
        0.5,
        Box::new(Metal::new(Vec3(0.8, 0.6, 0.2), 0.0)),
    )));
    world.list.push(Box::new(Sphere::new(
        Vec3(-1.0, 0.0, -1.0),
        0.5,
        Box::new(Dielectric::new(1.5)),
    )));
    world.list.push(Box::new(Sphere::new(
        Vec3(-1.0, 0.0, -1.0),
        -0.45,
        Box::new(Dielectric::new(1.5)),
    )));*/