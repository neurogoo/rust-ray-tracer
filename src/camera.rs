use std::f32::*;

use vector::*;
use ray::*;
use utils::*;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f32,
}

impl Camera {
    pub fn new(
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
        vfov: f32,
        aspect: f32,
        aperture: f32,
        focus_dist: f32,
    ) -> Camera {
        let lens_radius = aperture / 2.0;
        let theta = vfov * consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let origin = lookfrom;
        let w = unit_vector(lookfrom - lookat);
        let u = unit_vector(cross(&vup, &w));
        let v = cross(&w, &u);
        Camera {
            lower_left_corner: origin - u * focus_dist * half_width - v * focus_dist * half_height
                - w * focus_dist,
            horizontal: u * focus_dist * half_width * 2.0,
            vertical: v * focus_dist * half_height * 2.0,
            origin: origin,
            u: u,
            v: v,
            w: w,
            lens_radius: lens_radius,
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rd = random_in_unit_sphere() * self.lens_radius;
        let offset = self.u * rd.x() + self.v * rd.y();
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin - offset,
        )
    }
}
