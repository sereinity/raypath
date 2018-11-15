use rulinalg::vector::Vector;

use crate::ray::{Ray, HitRec};
use crate::sphere::random_in_unit_sphere;

pub enum Material {
    Lambertian(Vector<f64>),
}

impl Material {
    pub fn scatter<'a>(&self, _ray: &Ray, hitr: &'a HitRec) -> (&Vector<f64>, Ray<'a>) {
        match self {
            Material::Lambertian(attenuation) => {
                let target = &hitr.norm + random_in_unit_sphere();
                (&attenuation, Ray::new(&hitr.p, target))
            },
        }
    }
}
