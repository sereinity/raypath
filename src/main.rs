extern crate rulinalg;
extern crate rand;

extern crate raytracer;

use rulinalg::vector::Vector;
use rulinalg::norm::Euclidean;

use rand::prelude::*;

use raytracer::ray::{Ray, HitRec};
use raytracer::camera::Camera;
use raytracer::object::Object;
use raytracer::sphere::{Sphere, random_in_unit_sphere};

fn hit(obj_list: &Vec<Box<Object>>, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRec> {
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

fn color(r: &Ray, world: &Vec<Box<Object>>) -> Vector<f64> {
    match hit(world, r, 0.0001, std::f64::INFINITY) {
        Some(hit_rec) => {
            let target = &hit_rec.norm + random_in_unit_sphere();
            color(&Ray::new(&hit_rec.p, target), &world)*0.5
        }
        None => {
            let t = 0.5*unitize(&r.dire)[1] + 1.0;
            Vector::ones(3)*(1.0-t) + Vector::new(vec![0.5, 0.7, 1.0])*t
        }
    }
}

fn main() {
    let nx = 200;
    let ny = 100;
    let ns = 100;

    let world: Vec<Box<Object>> = vec![
        Box::new(Sphere {
            center: Vector::new(vec![0.0, 0.0, -1.0]),
            radius: 0.5,
        }),
        Box::new(Sphere {
            center: Vector::new(vec![0.0, -100.5, -1.0]),
            radius: 100.0,
        }),
    ];

    let cam = Camera::new();
    let mut rng = thread_rng();

    println!("P3\n{} {}\n255", nx, ny);
    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vector::new(vec![0.0, 0.0, 0.0]);
            for _ in 0..ns {
                let u = (i as f64 + rng.gen::<f64>()) / nx as f64;
                let v = (j as f64 + rng.gen::<f64>()) / ny as f64;
                let r = cam.get_ray(u, v);
                col += color(&r, &world);
            }
            let col = Vector::new(col.data().into_iter().map(|x| ((x/ns as f64).sqrt()*255.99) as usize).collect::<Vec<_>>());
            println!("{} {} {}", col[0], col[1], col[2]);
        }
    }
}

fn unitize(vect: &Vector<f64>) -> Vector<f64> {
    let norm = vect.norm(Euclidean);
    vect / norm
}
