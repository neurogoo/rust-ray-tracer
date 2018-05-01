use rand::{thread_rng, Rng};
use std::cmp::Ordering;

use ray::*;
use hitable::{HitRecord, Hitable};
use utils::{surrounding_box, Aabb};

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
    pub fn bounding_box(&self, t0: f32, t1: f32) -> Option<Aabb> {
        Some(self.bbox.clone())
    }

    pub fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
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
