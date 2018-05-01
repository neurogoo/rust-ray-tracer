use std::f32;

use hitable::{HitRecord, Hitable};
use utils::Aabb;
use vector::*;
use ray::*;

#[derive(Clone, Debug)]
pub struct RotateY {
    ptr: Box<Hitable>,
    sin_theta: f32,
    cos_theta: f32,
    hasbox: bool,
    bbox: Aabb,
}

impl RotateY {
    pub fn new(p: Hitable, angle: f32) -> RotateY {
        let radians = (f32::consts::PI / 180.0) * angle;
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let mut min = Vec3(f32::MAX, f32::MAX, f32::MAX);
        let mut max = Vec3(f32::MIN, f32::MIN, f32::MIN);
        let hasbox: bool;
        match p.bounding_box(0.0, 1.0) {
            Some(bbox) => {
                hasbox = true;
                for i in 0..2 {
                    for j in 0..2 {
                        for k in 0..2 {
                            let x = i as f32 * bbox.max().x() + (1.0 - i as f32) * bbox.min().x();
                            let y = j as f32 * bbox.max().y() + (1.0 - j as f32) * bbox.min().y();
                            let z = k as f32 * bbox.max().z() + (1.0 - k as f32) * bbox.min().z();
                            let new_x = cos_theta * x + sin_theta * z;
                            let new_z = -sin_theta * x + cos_theta * z;
                            let tester = Vec3(new_x, y, new_z);
                            for c in 0..3 {
                                if tester[c] > max[c] {
                                    max[c] = tester[c];
                                }
                                if tester[c] < min[c] {
                                    min[c] = tester[c];
                                }
                            }
                        }
                    }
                }
            }
            None => {
                hasbox = false;
            }
        };
        RotateY {
            ptr: Box::new(p),
            sin_theta: sin_theta,
            cos_theta: cos_theta,
            hasbox: hasbox,
            bbox: Aabb::new(min, max),
        }
    }

    pub fn hit(&self, r: &Ray, t0: f32, t1: f32) -> Option<HitRecord> {
        let mut origin = r.origin();
        let mut direction = r.direction();
        origin[0] = self.cos_theta * r.origin()[0] - self.sin_theta * r.origin()[2];
        origin[2] = self.sin_theta * r.origin()[0] + self.cos_theta * r.origin()[2];
        direction[0] = self.cos_theta * r.direction()[0] - self.sin_theta * r.direction()[2];
        direction[2] = self.sin_theta * r.direction()[0] + self.cos_theta * r.direction()[2];
        let rotated_r = Ray::new(origin, direction, r.time());
        match self.ptr.hit(&rotated_r, t0, t1) {
            Some(mut rec) => {
                let mut p = rec.p;
                let mut normal = rec.normal;
                p[0] = self.cos_theta * rec.p[0] + self.sin_theta * rec.p[2];
                p[2] = -self.sin_theta * rec.p[0] + self.cos_theta * rec.p[2];
                normal[0] = self.cos_theta * rec.normal[0] + self.sin_theta * rec.normal[2];
                normal[2] = -self.sin_theta * rec.normal[0] + self.cos_theta * rec.normal[2];
                rec.p = p;
                rec.normal = normal;
                return Some(rec);
            }
            None => {
                return None;
            }
        }
    }

    pub fn bounding_box(&self, t0: f32, t1: f32) -> Option<Aabb> {
        if self.hasbox {
            Some(self.bbox.clone())
        } else {
            None
        }
    }
}
