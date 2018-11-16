use rulinalg::vector::Vector;

use crate::unitize;
use crate::ray::{Ray, HitRec};
use crate::sphere::random_in_unit_sphere;

pub enum Material {
    Lambertian(Vector<f64>),
    Metal(Vector<f64>, f64),
}

impl Material {
    pub fn scatter<'a>(&self, ray: &Ray, hitr: &'a HitRec) -> Option<(&Vector<f64>, Ray<'a>)> {
        match self {
            Material::Lambertian(attenuation) => {
                let target = &hitr.norm + random_in_unit_sphere();
                Some((&attenuation, Ray::new(&hitr.p, target)))
            },
            Material::Metal(attenuation, fuzz) => {
                let reflected = reflect(unitize(&ray.dire), &hitr.norm);
                let scattered = Ray::new(&hitr.p, reflected + random_in_unit_sphere()*fuzz.min(1.0));
                if scattered.dire.dot(&hitr.norm) > 0.0 {
                    Some((&attenuation, scattered))
                } else {
                    None
                }
            },
        }
    }
}

fn reflect(vect: Vector<f64>, norm: &Vector<f64>) -> Vector<f64> {
    &vect - norm*2.0*vect.dot(norm)
}
