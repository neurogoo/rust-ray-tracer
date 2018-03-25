use rand::{thread_rng, Rng};
use std::cmp::Ordering;
use std::sync::Arc;
use std::fmt::Debug;

use vector::*;
use ray::*;
use material::*;
use utils::*;
use sphere::*;

pub struct HitRecord<'a> {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: &'a Material,
}

impl<'a> HitRecord<'a> {
    pub fn new(t: f32, p: Vec3, normal: Vec3, material: &'a Material) -> HitRecord {
        HitRecord {
            t: t,
            p: p,
            normal: normal,
            material: material,
        }
    }

    pub fn scatter(&self, r_in: &Ray) -> Scattered<(Ray, Vec3), (Ray, Vec3)> {
        scatter(&self.material, r_in, self)
    }
}

#[derive(Clone, Debug)]
pub enum Hitable {
    HitableList(HitableList),
    Sphere(Sphere),
    MovingSphere(MovingSphere),
    BvhNode(BvhNode),
}

impl Hitable {
    pub fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        match *self {
            Hitable::HitableList(ref hitable_list) => hitable_list.hit(r, t_min, t_max),
            Hitable::Sphere(ref sphere) => sphere.hit(r, t_min, t_max),
            Hitable::MovingSphere(ref moving_sphere) => moving_sphere.hit(r, t_min, t_max),
            Hitable::BvhNode(ref bvh_node) => bvh_node.hit(r, t_min, t_max),
        }
    }
    pub fn bounding_box(&self, t0: f32, t1: f32) -> Option<Aabb> {
        match *self {
            Hitable::HitableList(ref hitable_list) => hitable_list.bounding_box(t0, t1),
            Hitable::Sphere(ref sphere) => sphere.bounding_box(t0, t1),
            Hitable::MovingSphere(ref moving_sphere) => moving_sphere.bounding_box(t0, t1),
            Hitable::BvhNode(ref bvh_node) => bvh_node.bounding_box(t0, t1),
        }
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
