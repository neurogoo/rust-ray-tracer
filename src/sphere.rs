use std::sync::Arc;

use ray::*;
use hitable::*;
use vector::*;
use material::*;
use utils::*;

#[derive(Clone, Debug)]
pub struct Sphere {
    center: Vec3,
    radius: f32,
    material: Material,
}

impl Sphere {
    pub fn new(cen: Vec3, r: f32, material: Material) -> Sphere {
        Sphere {
            center: cen,
            radius: r,
            material: material,
        }
    }
}

impl Sphere {
    pub fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = dot(&r.direction(), &r.direction());
        let b = dot(&oc, &r.direction());
        let c = dot(&oc, &oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let mut temp = (-b - (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                let t = temp;
                let p = r.point_at_parameter(t);
                return Some(HitRecord::new(
                    t,
                    p,
                    (p - self.center) / self.radius,
                    &self.material,
                ));
            }
            temp = (-b + (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                let t = temp;
                let p = r.point_at_parameter(t);
                return Some(HitRecord::new(
                    t,
                    p,
                    (p - self.center) / self.radius,
                    &self.material,
                ));
            }
        }
        return None;
    }

    pub fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<Aabb> {
        Some(Aabb::new(
            self.center - Vec3(self.radius, self.radius, self.radius),
            self.center + Vec3(self.radius, self.radius, self.radius),
        ))
    }
}

#[derive(Clone, Debug)]
pub struct MovingSphere {
    center0: Vec3,
    center1: Vec3,
    time0: f32,
    time1: f32,
    radius: f32,
    material: Material,
}

impl MovingSphere {
    pub fn new(
        cen0: Vec3,
        cen1: Vec3,
        t0: f32,
        t1: f32,
        r: f32,
        material: Material,
    ) -> MovingSphere {
        MovingSphere {
            center0: cen0,
            center1: cen1,
            time0: t0,
            time1: t1,
            radius: r,
            material: material,
        }
    }

    pub fn center(&self, time: f32) -> Vec3 {
        self.center0
            + (self.center1 - self.center0) * ((time - self.time0) / (self.time1 - self.time0))
    }
}

impl MovingSphere {
    pub fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = r.origin() - self.center(r.time());
        let a = dot(&r.direction(), &r.direction());
        let b = dot(&oc, &r.direction());
        let c = dot(&oc, &oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let mut temp = (-b - (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                let t = temp;
                let p = r.point_at_parameter(t);
                return Some(HitRecord::new(
                    t,
                    p,
                    (p - self.center(r.time())) / self.radius,
                    &self.material,
                ));
            }
            temp = (-b + (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                let t = temp;
                let p = r.point_at_parameter(t);
                return Some(HitRecord::new(
                    t,
                    p,
                    (p - self.center(r.time())) / self.radius,
                    &self.material,
                ));
            }
        }
        return None;
    }

    pub fn bounding_box(&self, t0: f32, t1: f32) -> Option<Aabb> {
        let box0 = Aabb::new(
            self.center(t0) - Vec3(self.radius, self.radius, self.radius),
            self.center(t0) + Vec3(self.radius, self.radius, self.radius),
        );
        let box1 = Aabb::new(
            self.center(t1) - Vec3(self.radius, self.radius, self.radius),
            self.center(t1) + Vec3(self.radius, self.radius, self.radius),
        );
        Some(surrounding_box(box0, box1))
    }
}
