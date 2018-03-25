use rand::{thread_rng, Rng};
use std::fmt::Debug;

use ray::*;
use hitable::*;
use vector::*;
use utils::*;

pub enum Scattered<T, E> {
    Yes(T),
    No(E),
}

#[derive(Clone, Debug)]
pub enum Material {
    Labertian { albedo: Vec3 },
    Metal { albedo: Vec3, fuzz: f32 },
    Dielectric { ref_idx: f32 },
}

pub fn new_metal(a: Vec3, f: f32) -> Material {
    Material::Metal {
        albedo: a,
        fuzz: {
            if f < 1.0 {
                f
            } else {
                1.0
            }
        },
    }
}

pub fn new_labertian(albedo: Vec3) -> Material {
    Material::Labertian { albedo: albedo }
}

pub fn new_dielectric(ref_idx: f32) -> Material {
    Material::Dielectric { ref_idx: ref_idx }
}

pub fn scatter(
    material: &Material,
    r_in: &Ray,
    rec: &HitRecord,
) -> Scattered<(Ray, Vec3), (Ray, Vec3)> {
    match material {
        &Material::Labertian { albedo } => {
            let target = rec.p + rec.normal + random_in_unit_sphere();
            let scattered = Ray::new(rec.p, target - rec.p, r_in.time());
            let attenuation = albedo;
            return Scattered::Yes((scattered, attenuation));
        }
        &Material::Metal { albedo, fuzz } => {
            let reflected = reflect(&unit_vector(r_in.direction()), &rec.normal);
            let scattered = Ray::new(
                rec.p,
                reflected + random_in_unit_sphere() * fuzz,
                r_in.time(),
            );
            let attenuation = albedo;
            if dot(&scattered.direction(), &rec.normal) > 0.0 {
                Scattered::Yes((scattered, attenuation))
            } else {
                Scattered::No((scattered, attenuation))
            }
        }
        &Material::Dielectric { ref_idx } => {
            let outward_normal: Vec3;
            let reflected = reflect(&r_in.direction(), &rec.normal);
            let ni_over_nt: f32;
            let attenuation = Vec3(1.0, 1.0, 1.0);
            let mut refracted: Vec3 = Vec3(0.0, 0.0, 0.0);
            let reflect_prob: f32;
            let cosine: f32;
            let scattered: Ray;
            if dot(&r_in.direction(), &rec.normal) > 0.0 {
                outward_normal = rec.normal * (-1.0);
                ni_over_nt = ref_idx;
                cosine = ref_idx + dot(&r_in.direction(), &rec.normal) / r_in.direction().length();
            } else {
                outward_normal = rec.normal;
                ni_over_nt = 1.0 / ref_idx;
                cosine = -dot(&r_in.direction(), &rec.normal) / r_in.direction().length();
            }
            match refract(&r_in.direction(), &outward_normal, ni_over_nt) {
                Some(refr) => {
                    refracted = refr;
                    reflect_prob = schlick(cosine, ref_idx);
                }
                None => {
                    reflect_prob = 1.0;
                }
            }
            let mut rng = thread_rng();
            if rng.gen::<f32>() < reflect_prob {
                scattered = Ray::new(rec.p, reflected, r_in.time());
            } else {
                scattered = Ray::new(rec.p, refracted, r_in.time());
            }
            Scattered::Yes((scattered, attenuation))
        }
    }
}

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - *n * (dot(v, n) * 2.0)
}

pub fn refract(v: &Vec3, n: &Vec3, ni_over_nt: f32) -> Option<Vec3> {
    let uv = unit_vector(*v);
    let dt = dot(&uv, n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        Some((uv - *n * dt) * ni_over_nt - *n * discriminant.sqrt())
    } else {
        None
    }
}

pub fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}
