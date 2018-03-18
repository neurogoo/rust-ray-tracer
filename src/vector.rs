use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;

#[derive(Debug, Copy, Clone)]
pub struct Vec3(pub f32, pub f32, pub f32);

pub trait VecMath {
    fn x(&self) -> f32;
    fn y(&self) -> f32;
    fn z(&self) -> f32;
    fn r(&self) -> f32;
    fn g(&self) -> f32;
    fn b(&self) -> f32;

    fn length(&self) -> f32;
    fn squared_length(&self) -> f32;
    fn make_unit_vector(&mut self);
}

impl Vec3 {
    pub fn x(&self) -> f32 {
        self.0
    }
    pub fn y(&self) -> f32 {
        self.1
    }
    pub fn z(&self) -> f32 {
        self.2
    }
    pub fn r(&self) -> f32 {
        self.0
    }
    pub fn g(&self) -> f32 {
        self.1
    }
    pub fn b(&self) -> f32 {
        self.2
    }

    fn length(&self) -> f32 {
        (self.0 * self.0 + self.1 * self.1 + self.2 * self.2).sqrt()
    }
    pub fn squared_length(&self) -> f32 {
        (self.0 * self.0 + self.1 * self.1 + self.2 * self.2)
    }
    fn make_unit_vector(&mut self) {
        let k = 1.0 / self.length();
        self.0 = self.0 * k;
        self.1 = self.1 * k;
        self.2 = self.2 * k;
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, t: f32) -> Vec3 {
        Vec3(t * self.0, t * self.1, t * self.2)
    }
}

impl Div for Vec3 {
    type Output = Vec3;

    fn div(self, other: Vec3) -> Vec3 {
        Vec3(self.0 / other.0, self.1 / other.1, self.2 * other.2)
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, t: f32) -> Vec3 {
        Vec3(self.0 / t, self.1 / t, self.2 / t)
    }
}

pub fn unit_vector(v: Vec3) -> Vec3 {
    let length = v.length();
    v / length
}

pub fn dot(first: &Vec3, other: &Vec3) -> f32 {
    first.0 * other.0 + first.1 * other.1 + first.2 * other.2
}

pub fn cross(first: &Vec3, other: &Vec3) -> Vec3 {
    Vec3(
        first.1 * other.2 - first.2 * other.1,
        -(first.0 * other.2 - first.2 * other.0),
        first.0 * other.1 - first.1 * other.0,
    )
}
