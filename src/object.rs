use crate::ray::{Ray, HitRec};

pub trait Object {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRec>;
}
