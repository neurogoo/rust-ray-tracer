use vector::*;
use perlin::*;

#[derive(Clone, Debug)]
pub enum Texture {
    ConstantTexture(ConstantTexture),
    CheckedTexture(CheckerTexture),
    NoiseTexture(NoiseTexture),
    ImageTexture(ImageTexture),
}

impl Texture {
    pub fn value(&self, u: f32, v: f32, p: &Vec3) -> Vec3 {
        match *self {
            Texture::ConstantTexture(ref constant_texture) => constant_texture.value(u, v, p),
            Texture::CheckedTexture(ref checked_texture) => checked_texture.value(u, v, p),
            Texture::NoiseTexture(ref noise_texture) => noise_texture.value(u, v, p),
            Texture::ImageTexture(ref image_texture) => image_texture.value(u, v, p),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ImageTexture {
    data: Vec<u8>,
    nx: u32,
    ny: u32,
}

impl ImageTexture {
    pub fn new(pixels: Vec<u8>, nx: u32, ny: u32) -> ImageTexture {
        ImageTexture {
            data: pixels,
            nx: nx,
            ny: ny,
        }
    }

    pub fn value(&self, u: f32, v: f32, _p: &Vec3) -> Vec3 {
        let mut i: i32 = (u * self.nx as f32) as i32;
        let mut j: i32 = ((1.0 - v) * self.ny as f32 - 0.001) as i32;
        if i < 0 {
            i = 0;
        }
        if j < 0 {
            j = 0;
        }
        let mut i = i as u32;
        let mut j = j as u32;
        if i > self.nx - 1 {
            i = self.nx - 1
        }
        if j > self.ny - 1 {
            j = self.ny - 1
        }
        let r = (self.data[(3 * i + 3 * self.nx * j) as usize]) as f32 / 255.0;
        let g = (self.data[(3 * i + 3 * self.nx * j + 1) as usize]) as f32 / 255.0;
        let b = (self.data[(3 * i + 3 * self.nx * j + 2) as usize]) as f32 / 255.0;
        Vec3(r, g, b)
    }
}

#[derive(Clone, Debug)]
pub struct ConstantTexture {
    color: Vec3,
}

impl ConstantTexture {
    pub fn new(value: Vec3) -> ConstantTexture {
        ConstantTexture { color: value }
    }

    pub fn value(&self, _u: f32, _v: f32, _p: &Vec3) -> Vec3 {
        self.color
    }
}

#[derive(Clone, Debug)]
pub struct CheckerTexture {
    odd: Box<Texture>,
    even: Box<Texture>,
}

impl CheckerTexture {
    pub fn new(t0: Texture, t1: Texture) -> CheckerTexture {
        CheckerTexture {
            odd: Box::new(t0),
            even: Box::new(t1),
        }
    }

    pub fn value(&self, u: f32, v: f32, p: &Vec3) -> Vec3 {
        let sines = (10.0 * p.x()).sin() * (10.0 * p.y()).sin() * (10.0 * p.z()).sin();
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}

#[derive(Clone, Debug)]
pub struct NoiseTexture {
    noise: Perlin,
    scale: f32,
}

impl NoiseTexture {
    pub fn new(scale: f32) -> NoiseTexture {
        NoiseTexture {
            noise: Perlin::new(),
            scale: scale,
        }
    }

    pub fn value(&self, _u: f32, _v: f32, p: &Vec3) -> Vec3 {
        Vec3(1.0, 1.0, 1.0) * 0.5
            * (1.0 + (self.scale * p.z() + 10.0 * self.noise.turb(p, 7)).sin())
    }
}
