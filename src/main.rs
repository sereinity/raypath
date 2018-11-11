extern crate rulinalg;
extern crate rand;

use rulinalg::vector::Vector;
use rulinalg::norm::Euclidean;

use rand::prelude::*;

struct Ray<'a> {
    orig: &'a Vector<f64>,
    dire: Vector<f64>,
}

impl<'a> Ray<'a> {
    fn point_at_parameter(&self, param: f64) -> Vector<f64> {
        self.orig + &self.dire*param
    }
}

struct HitRec {
    t: f64,
    p: Vector<f64>,
    norm: Vector<f64>,
}

trait Object {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRec>;
}

struct Sphere {
    center: Vector<f64>,
    radius: f64,
}

impl Object for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRec> {
        let oc = ray.orig - &self.center;
        let a = ray.dire.dot(&ray.dire);
        let b = oc.dot(&ray.dire);
        let c = oc.dot(&oc) - &self.radius*&self.radius;
        let discriminent = b*b - a*c;
        return if discriminent >= 0.0 {
            let temp = (-b -discriminent.sqrt()) / a;
            if (temp <= t_max) & (temp >= t_min) {
                let point = ray.point_at_parameter(temp);
                let norm = unitize(&((&point - &self.center) / &self.radius));
                return Some(HitRec{
                    t: temp,
                    p: point,
                    norm: norm,
                });
            }
            let temp = (-b +discriminent.sqrt()) / a;
            if (temp <= t_max) & (temp >= t_min) {
                let point = ray.point_at_parameter(temp);
                let norm = unitize(&((&point - &self.center) / &self.radius));
                return Some(HitRec{
                    t: temp,
                    p: point,
                    norm: norm,
                });
            }
            None
        } else {
            None
        }
    }
}

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

struct Camera {
    lower_left_corner: Vector<f64>,
    horizontal: Vector<f64>,
    vertical: Vector<f64>,
    origin: Vector<f64>,
}

impl Camera {
    fn new() -> Camera {
        Camera {
            lower_left_corner: Vector::new(vec![-2.0, -1.0, -1.0]),
            horizontal: Vector::new(vec![4.0, 0.0, 0.0]),
            vertical: Vector::new(vec![0.0, 2.0, 0.0]),
            origin: Vector::new(vec![0.0, 0.0, 0.0]),
        }
    }

    fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray{
            orig: &self.origin,
            dire: &self.lower_left_corner + &self.horizontal*u + &self.vertical*v - &self.origin,
        }
    }
}

fn color(r: &Ray, world: &Vec<Box<Object>>) -> Vector<f64> {
    match hit(world, r, 0.0, std::f64::MAX) {
        Some(hit_rec) => {
            (hit_rec.norm + Vector::ones(3))*0.5
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
            col *= 255.99;
            col /= ns as f64;
            println!("{} {} {}", col[0] as usize, col[1] as usize, col[2] as usize);
        }
    }
}

fn unitize(vect: &Vector<f64>) -> Vector<f64> {
    let norm = vect.norm(Euclidean);
    vect / norm
}
