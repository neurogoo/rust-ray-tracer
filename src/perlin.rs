use rand::{thread_rng, Rng};
use std::fmt::{Debug, Formatter, Result};

use vector::*;

fn trilinear_interp(c: &[[[f32; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
    let mut accum = 0.0;
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let i_f32 = i as f32;
                let j_f32 = j as f32;
                let k_f32 = k as f32;
                accum = accum
                    + (i_f32 * u + (1.0 - i_f32) * (1.0 - u))
                        * (j_f32 * v + (1.0 - j_f32) * (1.0 - v))
                        * (k_f32 * w + (1.0 - k_f32) * (1.0 - w))
                        * c[i][j][k];
            }
        }
    }
    accum
}

fn perlin_interp(c: &[[[Vec3; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
    let uu = u * u * (3.0 - 2.0 * u);
    let vv = v * v * (3.0 - 2.0 * v);
    let ww = w * w * (3.0 - 2.0 * w);
    let mut accum = 0.0;
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let i_f32 = i as f32;
                let j_f32 = j as f32;
                let k_f32 = k as f32;
                let weight_v = Vec3(u - i_f32, v - j_f32, w - k_f32);
                accum = accum
                    + (i_f32 * uu + (1.0 - i_f32) * (1.0 - uu))
                        * (j_f32 * vv + (1.0 - j_f32) * (1.0 - vv))
                        * (k_f32 * ww + (1.0 - k_f32) * (1.0 - ww))
                        * dot(&c[i][j][k], &weight_v);
            }
        }
    }
    accum
}

#[derive(Clone)]
pub struct Perlin {
    perm_x: [u32; 256],
    perm_y: [u32; 256],
    perm_z: [u32; 256],
    ranvec: [Vec3; 256],
}

impl Debug for Perlin {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        write!(formatter, "Perlin noise")
    }
}

impl Perlin {
    pub fn new() -> Perlin {
        Perlin {
            perm_x: perlin_generate_perm(),
            perm_y: perlin_generate_perm(),
            perm_z: perlin_generate_perm(),
            ranvec: perlin_generate(),
        }
    }

    pub fn noise(&self, p: Vec3) -> f32 {
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();
        let i = p.x().floor() as usize;
        let j = p.y().floor() as usize;
        let k = p.z().floor() as usize;

        let mut c = [
            [
                [Vec3(0.0, 0.0, 0.0), Vec3(0.0, 0.0, 0.0)],
                [Vec3(0.0, 0.0, 0.0), Vec3(0.0, 0.0, 0.0)],
            ],
            [
                [Vec3(0.0, 0.0, 0.0), Vec3(0.0, 0.0, 0.0)],
                [Vec3(0.0, 0.0, 0.0), Vec3(0.0, 0.0, 0.0)],
            ],
        ];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.ranvec[(self.perm_x[(i + di) & 255]
                                                    ^ self.perm_y[(j + dj) & 255]
                                                    ^ self.perm_z[(k + dk) & 255])
                                                    as usize];
                }
            }
        }
        perlin_interp(&c, u, v, w)
    }

    pub fn turb(&self, p: &Vec3, depth: u32) -> f32 {
        let mut accum = 0.0;
        let mut temp_p = *p;
        let mut weight = 1.0;
        for _i in 0..depth {
            accum = accum + weight * self.noise(temp_p);
            weight = weight * 0.5;
            temp_p = temp_p * 2.0;
        }
        accum.abs()
    }
}

pub fn perlin_generate() -> [Vec3; 256] {
    let mut rng = thread_rng();
    let mut p: [Vec3; 256] = [Vec3(0.0, 0.0, 0.0); 256];
    for i in 0..256 {
        p[i] = unit_vector(Vec3(
            -1.0 + 2.0 * rng.gen::<f32>(),
            -1.0 + 2.0 * rng.gen::<f32>(),
            -1.0 + 2.0 * rng.gen::<f32>(),
        ));
    }
    p
}

pub fn permute(p: &mut [u32], n: u32) {
    let mut rng = thread_rng();
    for i in (1..n).rev() {
        let target = (rng.gen::<f32>() * (i + 1) as f32) as u32;
        let temp = p[i as usize];
        p[i as usize] = p[target as usize];
        p[target as usize] = temp;
    }
}

pub fn perlin_generate_perm() -> [u32; 256] {
    let mut p: [u32; 256] = [0; 256];
    for i in 0..256 {
        p[i as usize] = i;
    }
    permute(&mut p, 256);
    p
}
