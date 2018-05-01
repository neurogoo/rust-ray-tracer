use std::f32;
use rand::{thread_rng, Rng};

use hitable::{HitRecord, Hitable};
use utils::Aabb;
use vector::*;
use ray::*;
use material::*;
use texture::*;

#[derive(Clone, Debug)]
pub struct ConstantMedium {
    boundary: Box<Hitable>,
    density: f32,
    phase_function: Material,
}

impl ConstantMedium {
    pub fn new(b: Hitable, d: f32, a: Texture) -> ConstantMedium {
        ConstantMedium {
            boundary: Box::new(b),
            density: d,
            phase_function: Material::Isotropic(Isotropic::new(a)),
        }
    }

    pub fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut rng = thread_rng();
        let db = rng.gen::<f32>() < 0.00001;
        match self.boundary.hit(r, f32::MIN, f32::MAX) {
            Some(mut rec1) => match self.boundary.hit(r, rec1.t + 0.0001, f32::MAX) {
                Some(mut rec2) => {
                    if rec1.t < t_min {
                        rec1.t = t_min
                    }
                    if rec2.t > t_max {
                        rec2.t = t_max
                    }
                    if rec1.t >= rec2.t {
                        return None;
                    }
                    let distance_inside_boundary = (rec2.t - rec1.t) * r.direction().length();
                    let hit_distance = -(1.0 / self.density) * (rng.gen::<f32>()).ln();
                    if hit_distance < distance_inside_boundary {
                        let t = rec1.t + hit_distance / r.direction().length();
                        return Some(HitRecord {
                            t: t,
                            p: r.point_at_parameter(t),
                            normal: Vec3(1.0, 0.0, 0.0),
                            material: &self.phase_function,
                            u: 0.0,
                            v: 0.0,
                        });
                    }
                }
                None => {}
            },
            None => {}
        }
        None
    }

    pub fn bounding_box(&self, t0: f32, t1: f32) -> Option<Aabb> {
        self.boundary.bounding_box(t0, t1)
    }
}
