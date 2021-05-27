use rand::prelude::*;

use crate::ray::{HitRec, Ray};
use crate::sphere::random_in_unit_sphere;
use crate::Vec3;

pub enum Material {
    Lambertian(Vec3),
    Metal(Vec3, f64),
    Dielectric(f64),
}

impl Material {
    pub(crate) fn scatter(&self, ray: &Ray, hitr: &HitRec) -> Option<(Vec3, Ray)> {
        match self {
            Material::Lambertian(attenuation) => {
                let target = &hitr.norm + random_in_unit_sphere();
                Some((attenuation.clone(), Ray::new(hitr.p, target)))
            }
            Material::Metal(attenuation, fuzz) => {
                let reflected = reflect(ray.dire.normalize(), &hitr.norm);
                let scattered =
                    Ray::new(hitr.p, reflected + random_in_unit_sphere() * fuzz.min(1.0));
                if scattered.dire.dot(&hitr.norm) > 0.0 {
                    Some((attenuation.clone(), scattered))
                } else {
                    None
                }
            }
            Material::Dielectric(ref_idx) => {
                let mut rng = thread_rng();
                let reflected = reflect(ray.dire.clone(), &hitr.norm);
                let attenuation = Vec3::from_element(1.0);
                let (out_norm, ni_over_nt, cosine) = if ray.dire.dot(&hitr.norm) > 0.0 {
                    let cosine = ref_idx * ray.dire.dot(&hitr.norm) / ray.dire.norm();
                    (-&hitr.norm, ref_idx.clone(), cosine)
                } else {
                    let cosine = -ray.dire.dot(&hitr.norm) / ray.dire.norm();
                    (hitr.norm.clone(), 1.0 / ref_idx, cosine)
                };
                if let Some(refracted) = refract(&ray.dire, out_norm, ni_over_nt) {
                    if rng.gen::<f64>() < schlick(cosine, &ref_idx) {
                        Some((attenuation, Ray::new(hitr.p, reflected)))
                    } else {
                        Some((attenuation, Ray::new(hitr.p, refracted)))
                    }
                } else {
                    Some((attenuation, Ray::new(hitr.p, reflected)))
                }
            }
        }
    }
}

fn reflect(vect: Vec3, norm: &Vec3) -> Vec3 {
    &vect - norm * 2.0 * vect.dot(norm)
}

fn refract(vect: &Vec3, norm: Vec3, ni_over_nt: f64) -> Option<Vec3> {
    let unit = vect.normalize();
    let dt = unit.dot(&norm);
    let discriminent = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminent > 0.0 {
        Some((unit - &norm * dt) * ni_over_nt - &norm * discriminent.sqrt())
    } else {
        None
    }
}

/// Christophe Schlick approximation's to know reflectiveness by angle
fn schlick(cosine: f64, ref_idx: &f64) -> f64 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}
