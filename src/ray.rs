use indicatif::{ProgressBar, ProgressStyle};
use rand::prelude::*;

use crate::Vec3;
use crate::camera::Camera;
use crate::material::Material;
use crate::object::Object;

pub struct Ray {
    pub orig: Vec3,
    pub dire: Vec3,
}

impl Ray {
    pub(crate) fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray {
            orig: origin,
            dire: direction,
        }
    }

    pub fn point_at_parameter(&self, param: f64) -> Vec3 {
        self.orig + &self.dire*param
    }
}

pub struct HitRec<'a> {
    pub t: f64,
    pub p: Vec3,
    pub norm: Vec3,
    pub material: &'a Material,
}

pub(crate) fn hit<'a>(obj_list: &'a Vec<Box<dyn Object>>, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRec<'a>> {
    let mut hit_rec: Option<HitRec> = None;
    let mut closest_so_far = t_max;
    for object in obj_list {
        if let Some(temp_hit_rec) = object.hit(ray, t_min, closest_so_far) {
            closest_so_far = temp_hit_rec.t;
            hit_rec = Some(temp_hit_rec);
        }
    }
    hit_rec
}

fn color(r: &Ray, world: &Vec<Box<dyn Object>>, depth: usize) -> Vec3 {
    match hit(world, r, 0.0001, std::f64::INFINITY) {
        Some(hit_rec) => {
            if depth < 50 {
                if let Some((attenuation, scattered)) = hit_rec.material.scatter(r, &hit_rec) {
                    color(&scattered, &world, depth+1).component_mul(&attenuation)
                } else {
                    Vec3::zeros()
                }
            } else {
                Vec3::zeros()
            }
        }
        None => {
            let t = 0.5*r.dire.normalize()[1] + 1.0;
            Vec3::from_element(1.0)*(1.0-t) + Vec3::new(0.5, 0.7, 1.0)*t
        }
    }
}

pub fn render(scene: Vec<Box<dyn Object>>, camera: Camera, nx: usize, ny: usize, ns: usize) -> Vec<u8> {
    let bar = ProgressBar::new((nx*ny) as u64);
    bar.set_message("Rendering");
    bar.set_style(ProgressStyle::default_bar()
                  .template("{msg} {wide_bar} eta: {eta} {pos:>7}/{len:7}")
        );
    let mut rng = thread_rng();
    let mut pixs = Vec::with_capacity(nx*ny*4);

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..ns {
                let u = (i as f64 + rng.gen::<f64>()) / nx as f64;
                let v = (j as f64 + rng.gen::<f64>()) / ny as f64;
                let r = camera.get_ray(u, v);
                col += color(&r, &scene, 0);
            }
            pixs.extend(col.map(|x| ((x/ns as f64).sqrt()*(u8::max_value() as f64)) as u8).as_slice());
            pixs.push(u8::max_value());
            bar.inc(1);
        }
    }
    bar.finish_and_clear();

    pixs
}
