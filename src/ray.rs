use vector::*;

pub struct Ray {
    a: Vec3,
    b: Vec3,
    t: f32,
}

impl Ray {
    pub fn new(a: Vec3, b: Vec3, t: f32) -> Ray {
        Ray { a: a, b: b, t: t }
    }

    pub fn origin(&self) -> Vec3 {
        self.a
    }

    pub fn direction(&self) -> Vec3 {
        self.b
    }

    pub fn time(&self) -> f32 {
        self.t
    }

    pub fn point_at_parameter(&self, t: f32) -> Vec3 {
        self.a + self.b * t
    }
}
