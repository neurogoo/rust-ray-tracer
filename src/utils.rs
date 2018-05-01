use std::f32;
use rand::{thread_rng, Rng};
use std::path::Path;
use stb_image::image;

use vector::Vec3;
use hitable::*;
use material::*;
use sphere::*;
use ray::*;
use texture::*;
use camera::Camera;

pub fn cornell_box(nx: u32, ny: u32) -> (Camera, Vec<Hitable>) {
    let mut list: Vec<Hitable> = Vec::new();
    let red = new_labertian(new_constant_texture(Vec3(0.65, 0.05, 0.05)));
    let white = new_labertian(new_constant_texture(Vec3(0.73, 0.73, 0.73)));
    let green = new_labertian(new_constant_texture(Vec3(0.12, 0.45, 0.15)));
    let light = new_diffuce(new_constant_texture(Vec3(15.0, 15.0, 15.0)));
    list.push(new_flip_normals(new_yzrect(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        green,
    )));
    list.push(new_yzrect(0.0, 555.0, 0.0, 555.0, 0.0, red));
    list.push(new_xzrect(213.0, 343.0, 227.0, 332.0, 554.0, light));
    list.push(new_flip_normals(new_xzrect(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));
    list.push(new_xzrect(0.0, 555.0, 0.0, 555.0, 0.0, white.clone()));
    list.push(new_flip_normals(new_xyrect(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));

    list.push(new_translate(
        new_rotate_y(
            new_box_(
                Vec3(0.0, 0.0, 0.0),
                Vec3(165.0, 165.0, 165.0),
                white.clone(),
            ),
            -18.0,
        ),
        Vec3(130.0, 0.0, 65.0),
    ));
    list.push(new_translate(
        new_rotate_y(
            new_box_(
                Vec3(0.0, 0.0, 0.0),
                Vec3(165.0, 330.0, 165.0),
                white.clone(),
            ),
            15.0,
        ),
        Vec3(265.0, 0.0, 295.0),
    ));

    let lookfrom = Vec3(278.0, 278.0, -800.0);
    let lookat = Vec3(278.0, 278.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.0;
    let vfov = 40.0;
    let cam = Camera::new(
        lookfrom,
        lookat,
        Vec3(0.0, 1.0, 0.0),
        vfov,
        nx as f32 / ny as f32,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );
    (cam, list)
}

pub fn simple_light() -> Vec<Hitable> {
    let pertext = Texture::NoiseTexture(NoiseTexture::new(4.0));
    let mut list: Vec<Hitable> = Vec::new();
    list.push(new_sphere(
        Vec3(0.0, -1000.0, 0.0),
        1000.0,
        new_labertian(pertext.clone()),
    ));
    list.push(new_sphere(
        Vec3(0.0, 2.0, 0.0),
        2.0,
        new_labertian(pertext.clone()),
    ));
    // list.push(new_sphere(
    //     Vec3(0.0, 7.0, 0.0),
    //     2.0,
    //     new_diffuce(Texture::ConstantTexture(ConstantTexture::new(Vec3(
    //         4.0,
    //         4.0,
    //         4.0,
    //     )))),
    // ));
    list.push(new_xyrect(
        3.0,
        5.0,
        1.0,
        3.0,
        -2.0,
        new_diffuce(Texture::ConstantTexture(ConstantTexture::new(Vec3(
            4.0,
            4.0,
            4.0,
        )))),
    ));
    list
}

pub fn new_rotate_y(hitable: Hitable, angle: f32) -> Hitable {
    Hitable::RotateY(RotateY::new(hitable, angle))
}

pub fn new_translate(hitable: Hitable, displacement: Vec3) -> Hitable {
    Hitable::Translate(Translate::new(hitable, displacement))
}

pub fn new_flip_normals(hitable: Hitable) -> Hitable {
    Hitable::FlipNormals(FlipNormals::new(hitable))
}

pub fn new_diffuce(albedo: Texture) -> Material {
    Material::DiffuceLight(DiffuceLight::new(albedo))
}

pub fn new_xyrect(x0: f32, x1: f32, y0: f32, y1: f32, k: f32, material: Material) -> Hitable {
    Hitable::XYRect(XYRect::new(x0, x1, y0, y1, k, material))
}

pub fn new_xzrect(x0: f32, x1: f32, z0: f32, z1: f32, k: f32, material: Material) -> Hitable {
    Hitable::XZRect(XZRect::new(x0, x1, z0, z1, k, material))
}

pub fn new_yzrect(y0: f32, y1: f32, z0: f32, z1: f32, k: f32, material: Material) -> Hitable {
    Hitable::YZRect(YZRect::new(y0, y1, z0, z1, k, material))
}

pub fn new_box_(p0: Vec3, p1: Vec3, ptr: Material) -> Hitable {
    Hitable::Box_(Box_::new(p0, p1, ptr))
}

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

fn new_sphere(cen: Vec3, r: f32, material: Material) -> Hitable {
    Hitable::Sphere(Sphere::new(cen, r, material))
}

fn new_labertian(albedo: Texture) -> Material {
    Material::Labertian(Labertian::new(albedo))
}

fn new_constant_texture(v: Vec3) -> Texture {
    Texture::ConstantTexture(ConstantTexture::new(v))
}

fn new_labertian_image(path: &str) -> Material {
    let path = Path::new(path);
    let result = image::load(path);
    let data: Vec<u8>;
    let nx: usize;
    let ny: usize;
    match result {
        image::LoadResult::ImageU8(image) => {
            data = image.data;
            nx = image.width;
            ny = image.height;
        }
        _ => panic!("Couldn't find data"),
    }
    Material::Labertian(Labertian::new(Texture::ImageTexture(ImageTexture::new(
        data,
        nx as u32,
        ny as u32,
    ))))
}

pub fn two_perlin_spheres() -> Vec<Hitable> {
    let pertext = Texture::NoiseTexture(NoiseTexture::new(1.5));
    let mut hitables: Vec<Hitable> = Vec::new();
    hitables.push(new_sphere(
        Vec3(0.0, -1000.0, 0.0),
        1000.0,
        new_labertian(pertext.clone()),
    ));
    hitables.push(new_sphere(
        Vec3(0.0, 2.0, 0.0),
        2.0,
        new_labertian_image("/home/tokuogum/Rust/rust-ray-tracer/GreatestEarth.jpg"),
    ));
    /*    hitables.push(new_sphere(
        Vec3(0.0, 2.0, 0.0),
        2.0,
        new_labertian(pertext.clone()),
));*/

    hitables
}

pub fn random_scene() -> Vec<Hitable> {
    let mut rng = thread_rng();
    let n = 50000;
    let mut world: Vec<Hitable> = Vec::new();
    let checker = Texture::CheckedTexture(CheckerTexture::new(
        Texture::ConstantTexture(ConstantTexture::new(Vec3(0.2, 0.3, 0.1))),
        Texture::ConstantTexture(ConstantTexture::new(Vec3(0.9, 0.9, 0.9))),
    ));
    world.push(Hitable::Sphere(Sphere::new(
        Vec3(0.0, -1000.0, 0.0),
        1000.0,
        Material::Labertian(Labertian::new(checker)),
    )));
    for a in -10..10 {
        for b in -10..10 {
            let choose_mat = rng.gen::<f32>();
            let center = Vec3(
                a as f32 + 0.9 * rng.gen::<f32>(),
                0.2,
                b as f32 + 0.9 * rng.gen::<f32>(),
            );
            if (center - Vec3(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    world.push(Hitable::MovingSphere(MovingSphere::new(
                        center,
                        center + Vec3(0.0, 0.5 * rng.gen::<f32>(), 0.0),
                        0.0,
                        1.0,
                        0.2,
                        Material::Labertian(Labertian::new(Texture::ConstantTexture(
                            ConstantTexture::new(Vec3(
                                rng.gen::<f32>() * rng.gen::<f32>(),
                                rng.gen::<f32>() * rng.gen::<f32>(),
                                rng.gen::<f32>() * rng.gen::<f32>(),
                            )),
                        ))),
                    )))
                } else if choose_mat < 0.95 {
                    world.push(Hitable::Sphere(Sphere::new(
                        center,
                        0.2,
                        Material::Metal(Metal::new(
                            Vec3(
                                0.5 * (1.0 + rng.gen::<f32>()),
                                0.5 * (1.0 + rng.gen::<f32>()),
                                0.5 * (1.0 + rng.gen::<f32>()),
                            ),
                            0.5 * rng.gen::<f32>(),
                        )),
                    )))
                } else {
                    world.push(Hitable::Sphere(Sphere::new(
                        center,
                        0.2,
                        Material::Dielectric(Dielectric::new(1.5)),
                    )))
                }
            }
        }
    }
    world.push(Hitable::Sphere(Sphere::new(
        Vec3(0.0, 1.0, 0.0),
        1.0,
        Material::Dielectric(Dielectric::new(1.5)),
    )));
    world.push(Hitable::Sphere(Sphere::new(
        Vec3(-4.0, 1.0, 0.0),
        1.0,
        Material::Labertian(Labertian::new(Texture::ConstantTexture(
            ConstantTexture::new((Vec3(0.4, 0.2, 0.1))),
        ))),
    )));
    world.push(Hitable::Sphere(Sphere::new(
        Vec3(4.0, 1.0, 0.0),
        1.0,
        Material::Metal(Metal::new(Vec3(0.7, 0.6, 0.5), 0.0)),
    )));
    world
}

#[inline]
pub fn ffmin(a: f32, b: f32) -> f32 {
    if a < b {
        a
    } else {
        b
    }
}

#[inline]
pub fn ffmax(a: f32, b: f32) -> f32 {
    if a > b {
        a
    } else {
        b
    }
}

#[derive(Debug, Clone)]
pub struct Aabb {
    _min: Vec3,
    _max: Vec3,
}

impl Aabb {
    pub fn new(a: Vec3, b: Vec3) -> Aabb {
        Aabb { _min: a, _max: b }
    }

    pub fn min(&self) -> Vec3 {
        self._min
    }

    pub fn max(&self) -> Vec3 {
        self._max
    }

    pub fn hit(&self, r: &Ray, tmin: f32, tmax: f32) -> bool {
        for a in 0..3 {
            let t0 = ffmin(
                (self._min[a] - r.origin()[a]) / r.direction()[a],
                (self._max[a] - r.origin()[a]) / r.direction()[a],
            );
            let t1 = ffmax(
                (self._min[a] - r.origin()[a]) / r.direction()[a],
                (self._max[a] - r.origin()[a]) / r.direction()[a],
            );
            let tmin = ffmax(t0, tmin);
            let tmax = ffmin(t1, tmax);
            if tmax <= tmin {
                return false;
            }
        }
        true
    }
}

pub fn surrounding_box(box0: Aabb, box1: Aabb) -> Aabb {
    Aabb::new(
        Vec3(
            ffmin(box0.min().x(), box1.min().x()),
            ffmin(box0.min().y(), box1.min().y()),
            ffmin(box0.min().z(), box1.min().z()),
        ),
        Vec3(
            ffmax(box0.max().x(), box1.max().x()),
            ffmax(box0.max().y(), box1.max().y()),
            ffmax(box0.max().z(), box1.max().z()),
        ),
    )
}

pub fn get_sphere_uv(p: &Vec3) -> (f32, f32) {
    let phi = p.z().atan2(p.x());
    let theta = p.y().asin();
    (
        1.0 - (phi + f32::consts::PI) / (2.0 * f32::consts::PI),
        (theta + f32::consts::PI / 2.0) / f32::consts::PI,
    )
}

/*let mut world = HitableList::new();
    let r = (f32::consts::PI / 4.0).cos();
    world
        .list
        .push(Box::new(Sphere::new(Vec3(-r, 0.0, -1.0), r, &lam1test)));
    world
        .list
        .push(Box::new(Sphere::new(Vec3(r, 0.0, -1.0), r, &lam2test)));
    world.list.push(Box::new(Sphere::new(
        Vec3(0.0, 0.0, -1.0),
        0.5,
        Box::new(Labertian::new(Vec3(0.1, 0.2, 0.5))),
    )));
    world.list.push(Box::new(Sphere::new(
        Vec3(0.0, -100.5, -1.0),
        100.0,
        Box::new(Labertian::new(Vec3(0.8, 0.8, 0.0))),
    )));
    world.list.push(Box::new(Sphere::new(
        Vec3(1.0, 0.0, -1.0),
        0.5,
        Box::new(Metal::new(Vec3(0.8, 0.6, 0.2), 0.0)),
    )));
    world.list.push(Box::new(Sphere::new(
        Vec3(-1.0, 0.0, -1.0),
        0.5,
        Box::new(Dielectric::new(1.5)),
    )));
    world.list.push(Box::new(Sphere::new(
        Vec3(-1.0, 0.0, -1.0),
        -0.45,
        Box::new(Dielectric::new(1.5)),
    )));*/
