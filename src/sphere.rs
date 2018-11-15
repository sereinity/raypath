use rulinalg::vector::Vector;
use crate::object::Object;
use crate::ray::{Ray, HitRec};
use crate::material::Material;

use rand::prelude::*;

pub struct Sphere {
    pub center: Vector<f64>,
    pub radius: f64,
    pub material: Material,
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
                let norm = (&point - &self.center) / &self.radius;
                return Some(HitRec{
                    t: temp,
                    p: point,
                    norm: norm,
                    material: &self.material,
                });
            }
            let temp = (-b +discriminent.sqrt()) / a;
            if (temp <= t_max) & (temp >= t_min) {
                let point = ray.point_at_parameter(temp);
                let norm = (&point - &self.center) / &self.radius;
                return Some(HitRec{
                    t: temp,
                    p: point,
                    norm: norm,
                    material: &self.material,
                });
            }
            None
        } else {
            None
        }
    }
}

pub fn random_in_unit_sphere() -> Vector<f64> {
    let mut rng = thread_rng();
    loop {
        // Random taken from a cube of 2x2x2 centered on 1x1x1
        let mut p = Vector::new(vec![rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>()])*2.0;
        p -= Vector::ones(3);
        if p.dot(&p) < 1.0 {  // Is it in a sphere?
            return p;
        }
    }
}
