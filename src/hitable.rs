use vector::*;
use ray::*;
use material::*;

pub struct HitRecord<'a> {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: &'a Material,
}

impl<'a> HitRecord<'a> {
    pub fn new(t: f32, p: Vec3, normal: Vec3, material: &'a Material) -> HitRecord<'a> {
        HitRecord {
            t: t,
            p: p,
            normal: normal,
            material: material,
        }
    }

    pub fn scatter(&self, r_in: &Ray) -> Scattered<(Ray, Vec3), (Ray, Vec3)> {
        self.material.scatter(r_in, self)
    }
}

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct HitableList<'a> {
    pub list: Vec<Box<Hitable + 'a>>,
}

impl<'a> HitableList<'a> {
    pub fn new() -> HitableList<'a> {
        HitableList { list: Vec::new() }
    }
}

impl<'a> Hitable for HitableList<'a> {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
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
        return hit_anything;
    }
}
