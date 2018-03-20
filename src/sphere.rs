use ray::*;
use hitable::*;
use vector::*;
use material::*;

pub struct Sphere {
    center: Vec3,
    radius: f32,
    material: Box<Material>,
}

impl Sphere {
    pub fn new(cen: Vec3, r: f32, material: Box<Material>) -> Sphere {
        Sphere {
            center: cen,
            radius: r,
            material: material,
        }
    }
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
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
                    &*self.material,
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
                    &*self.material,
                ));
            }
        }
        return None;
    }
}
