use std::f32;
use rand::{thread_rng, Rng};
use vector::*;

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
