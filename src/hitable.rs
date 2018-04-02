use rand::{thread_rng, Rng};
use std::cmp::Ordering;

use vector::Vec3;
use ray::*;
use material::*;
use utils::*;
use sphere::*;

pub struct HitRecord<'a> {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: &'a Material,
    pub u: f32,
    pub v: f32,
}

impl<'a> HitRecord<'a> {
    pub fn new(t: f32, p: Vec3, normal: Vec3, material: &'a Material, u: f32, v: f32) -> HitRecord {
        HitRecord {
            t: t,
            p: p,
            normal: normal,
            material: material,
            u: u,
            v: v,
        }
    }

    pub fn scatter(&self, r_in: &Ray) -> Scattered<(Ray, Vec3)> {
        self.material.scatter(r_in, self)
    }
}

#[derive(Clone, Debug)]
pub enum Hitable {
    HitableList(HitableList),
    Sphere(Sphere),
    MovingSphere(MovingSphere),
    BvhNode(BvhNode),
    XYRect(XYRect),
    XZRect(XZRect),
    YZRect(YZRect),
    FlipNormals(FlipNormals),
}

impl Hitable {
    pub fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        match *self {
            Hitable::HitableList(ref hitable_list) => hitable_list.hit(r, t_min, t_max),
            Hitable::Sphere(ref sphere) => sphere.hit(r, t_min, t_max),
            Hitable::MovingSphere(ref moving_sphere) => moving_sphere.hit(r, t_min, t_max),
            Hitable::BvhNode(ref bvh_node) => bvh_node.hit(r, t_min, t_max),
            Hitable::XYRect(ref xyrect) => xyrect.hit(r, t_min, t_max),
            Hitable::XZRect(ref xzrect) => xzrect.hit(r, t_min, t_max),
            Hitable::YZRect(ref yzrect) => yzrect.hit(r, t_min, t_max),
            Hitable::FlipNormals(ref flip_normals) => flip_normals.hit(r, t_min, t_max),
        }
    }
    pub fn bounding_box(&self, t0: f32, t1: f32) -> Option<Aabb> {
        match *self {
            Hitable::HitableList(ref hitable_list) => hitable_list.bounding_box(t0, t1),
            Hitable::Sphere(ref sphere) => sphere.bounding_box(t0, t1),
            Hitable::MovingSphere(ref moving_sphere) => moving_sphere.bounding_box(t0, t1),
            Hitable::BvhNode(ref bvh_node) => bvh_node.bounding_box(t0, t1),
            Hitable::XYRect(ref xyrect) => xyrect.bounding_box(t0, t1),
            Hitable::XZRect(ref xzrect) => xzrect.bounding_box(t0, t1),
            Hitable::YZRect(ref yzrect) => yzrect.bounding_box(t0, t1),
            Hitable::FlipNormals(ref flip_normals) => flip_normals.bounding_box(t0, t1),
        }
    }
}

#[derive(Clone, Debug)]
pub struct FlipNormals {
    ptr: Box<Hitable>,
}

impl FlipNormals {
    pub fn new(p: Hitable) -> FlipNormals {
        FlipNormals { ptr: Box::new(p) }
    }
    fn hit(&self, r: &Ray, t0: f32, t1: f32) -> Option<HitRecord> {
        match self.ptr.hit(r, t0, t1) {
            Some(mut rec) => {
                rec.normal = rec.normal * -1.0;
                return Some(rec);
            }
            None => None,
        }
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<Aabb> {
        self.ptr.bounding_box(t0, t1)
    }
}

#[derive(Clone, Debug)]
pub struct YZRect {
    mp: Material,
    y0: f32,
    y1: f32,
    z0: f32,
    z1: f32,
    k: f32,
}

impl YZRect {
    pub fn new(y0: f32, y1: f32, z0: f32, z1: f32, k: f32, mp: Material) -> YZRect {
        YZRect {
            mp: mp,
            y0: y0,
            y1: y1,
            z0: z0,
            z1: z1,
            k: k,
        }
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<Aabb> {
        Some(Aabb::new(
            Vec3(self.k - 0.0001, self.y0, self.z0),
            Vec3(self.k + 0.0001, self.y1, self.z1),
        ))
    }

    fn hit(&self, r: &Ray, t0: f32, t1: f32) -> Option<HitRecord> {
        let t = (self.k - r.origin().x()) / r.direction().x();
        if t < t0 || t > t1 {
            return None;
        }
        let y = r.origin().y() + t * r.direction().y();
        let z = r.origin().z() + t * r.direction().z();
        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return None;
        }
        Some(HitRecord::new(
            t,
            r.point_at_parameter(t),
            Vec3(1.0, 0.0, 0.0),
            &self.mp,
            (y - self.y0) / (self.y1 - self.y0),
            (z - self.z0) / (self.z1 - self.z0),
        ))
    }
}

#[derive(Clone, Debug)]
pub struct XZRect {
    mp: Material,
    x0: f32,
    x1: f32,
    z0: f32,
    z1: f32,
    k: f32,
}

impl XZRect {
    pub fn new(x0: f32, x1: f32, z0: f32, z1: f32, k: f32, mp: Material) -> XZRect {
        XZRect {
            mp: mp,
            x0: x0,
            x1: x1,
            z0: z0,
            z1: z1,
            k: k,
        }
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<Aabb> {
        Some(Aabb::new(
            Vec3(self.x0, self.k - 0.0001, self.z0),
            Vec3(self.x1, self.k + 0.0001, self.z1),
        ))
    }

    fn hit(&self, r: &Ray, t0: f32, t1: f32) -> Option<HitRecord> {
        let t = (self.k - r.origin().y()) / r.direction().y();
        if t < t0 || t > t1 {
            return None;
        }
        let x = r.origin().x() + t * r.direction().x();
        let z = r.origin().z() + t * r.direction().z();
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return None;
        }
        Some(HitRecord::new(
            t,
            r.point_at_parameter(t),
            Vec3(0.0, 1.0, 0.0),
            &self.mp,
            (x - self.x0) / (self.x1 - self.x0),
            (z - self.z0) / (self.z1 - self.z0),
        ))
    }
}

#[derive(Clone, Debug)]
pub struct XYRect {
    mp: Material,
    x0: f32,
    x1: f32,
    y0: f32,
    y1: f32,
    k: f32,
}

impl XYRect {
    pub fn new(x0: f32, x1: f32, y0: f32, y1: f32, k: f32, mp: Material) -> XYRect {
        XYRect {
            mp: mp,
            x0: x0,
            x1: x1,
            y0: y0,
            y1: y1,
            k: k,
        }
    }

    fn hit(&self, r: &Ray, t0: f32, t1: f32) -> Option<HitRecord> {
        let t = (self.k - r.origin().z()) / r.direction().z();
        if t < t0 || t > t1 {
            return None;
        }
        let x = r.origin().x() + t * r.direction().x();
        let y = r.origin().y() + t * r.direction().y();
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }
        Some(HitRecord::new(
            t,
            r.point_at_parameter(t),
            Vec3(0.0, 0.0, 1.0),
            &self.mp,
            (x - self.x0) / (self.x1 - self.x0),
            (y - self.y0) / (self.y1 - self.y0),
        ))
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<Aabb> {
        Some(Aabb::new(
            Vec3(self.x0, self.y0, self.k - 0.0001),
            Vec3(self.x1, self.y1, self.k + 0.0001),
        ))
    }
}

#[derive(Clone, Debug)]
pub struct HitableList {
    pub list: Vec<Hitable>,
}

impl HitableList {
    pub fn new(vector: Vec<Hitable>) -> HitableList {
        HitableList { list: vector }
    }
    pub fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut hit_anything = None;
        let mut closest_so_far = t_max;
        for i in self.list.iter() {
            match i.hit(r, t_min, closest_so_far) {
                Some(temp_rec) => {
                    closest_so_far = temp_rec.t;
                    hit_anything = Some(temp_rec);
                }
                None => {}
            }
        }
        hit_anything
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<Aabb> {
        if self.list.len() < 1 {
            return None;
        }

        let mut temp_box: Aabb;
        let first_true = self.list[0].bounding_box(t0, t1);
        match first_true {
            Some(bbox) => temp_box = bbox,
            None => return None,
        }
        for i in 1..(self.list.len()) {
            match self.list[i].bounding_box(t0, t1) {
                Some(bbox) => temp_box = surrounding_box(temp_box, bbox),
                None => return None,
            }
        }
        Some(temp_box)
    }
}

fn box_x_compare(a: &Hitable, b: &Hitable) -> Ordering {
    match (a.bounding_box(0.0, 0.0), b.bounding_box(0.0, 0.0)) {
        (None, _) => println!("No bounding box in bcvh_node construstor"),
        (_, None) => println!("No bounding box in bcvh_node construstor"),
        (Some(box_left), Some(box_right)) => if box_left.min().x() - box_right.min().x() < 0.0 {
            return Ordering::Less;
        },
    }
    Ordering::Greater
}

fn box_y_compare(a: &Hitable, b: &Hitable) -> Ordering {
    match (a.bounding_box(0.0, 0.0), b.bounding_box(0.0, 0.0)) {
        (None, _) => println!("No bounding box in bcvh_node construstor"),
        (_, None) => println!("No bounding box in bcvh_node construstor"),
        (Some(box_left), Some(box_right)) => if box_left.min().y() - box_right.min().y() < 0.0 {
            return Ordering::Less;
        },
    }
    Ordering::Greater
}

fn box_z_compare(a: &Hitable, b: &Hitable) -> Ordering {
    match (a.bounding_box(0.0, 0.0), b.bounding_box(0.0, 0.0)) {
        (None, _) => println!("No bounding box in bcvh_node construstor"),
        (_, None) => println!("No bounding box in bcvh_node construstor"),
        (Some(box_left), Some(box_right)) => if box_left.min().z() - box_right.min().z() < 0.0 {
            return Ordering::Less;
        },
    }
    Ordering::Greater
}

#[derive(Clone, Debug)]
pub struct BvhNode {
    left: Box<Hitable>,
    right: Box<Hitable>,
    bbox: Aabb,
}

impl BvhNode {
    pub fn new(hitable: &mut [Hitable], time0: f32, time1: f32) -> BvhNode {
        let left: Box<Hitable>;
        let right: Box<Hitable>;
        let n = hitable.len();
        let mut rng = thread_rng();
        let axis = (3.0 * rng.gen::<f32>()) as u32;
        match axis {
            0 => hitable.sort_by(|a, b| box_x_compare(a, b)),
            1 => hitable.sort_by(|a, b| box_y_compare(a, b)),
            _ => hitable.sort_by(|a, b| box_z_compare(a, b)),
        }
        if n == 1 {
            left = Box::new(hitable[0].clone());
            right = Box::new(hitable[0].clone());
        } else if n == 2 {
            left = Box::new(hitable[0].clone());
            right = Box::new(hitable[1].clone());
        } else {
            let (vec_start, vec_end) = hitable.split_at_mut(n / 2);
            left = Box::new(Hitable::BvhNode(BvhNode::new(vec_start, time0, time1)));
            right = Box::new(Hitable::BvhNode(BvhNode::new(vec_end, time0, time1)));
        }
        match (
            left.bounding_box(time0, time1),
            right.bounding_box(time0, time1),
        ) {
            (None, _) => panic!("No bounding box in bcvh_node construstor"),
            (_, None) => panic!("No bounding box in bcvh_node construstor"),
            (Some(box_left), Some(box_right)) => BvhNode {
                left: left,
                right: right,
                bbox: surrounding_box(box_left, box_right),
            },
        }
    }
}

impl BvhNode {
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<Aabb> {
        Some(self.bbox.clone())
    }

    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if self.bbox.hit(r, t_min, t_max) {
            let hit_left = self.left.hit(r, t_min, t_max);
            let hit_right = self.right.hit(r, t_min, t_max);
            match (hit_left, hit_right) {
                (Some(left_rec), Some(right_rec)) => {
                    if left_rec.t < right_rec.t {
                        return Some(left_rec);
                    } else {
                        return Some(right_rec);
                    }
                }
                (Some(left_rec), _) => return Some(left_rec),
                (_, Some(right_rec)) => return Some(right_rec),
                (None, None) => return None,
            }
        }
        None
    }
}
