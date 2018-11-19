use crate::Vec3;
use crate::material::Material;

pub struct Ray {
    pub orig: Vec3,
    pub dire: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
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
