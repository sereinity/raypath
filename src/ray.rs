use rulinalg::vector::Vector;

pub struct Ray<'a> {
    pub orig: &'a Vector<f64>,
    pub dire: Vector<f64>,
}

impl<'a> Ray<'a> {
    pub fn new(origin: &Vector<f64>, direction: Vector<f64>) -> Ray {
        Ray {
            orig: origin,
            dire: direction,
        }
    }

    pub fn point_at_parameter(&self, param: f64) -> Vector<f64> {
        self.orig + &self.dire*param
    }
}

pub struct HitRec {
    pub t: f64,
    pub p: Vector<f64>,
    pub norm: Vector<f64>,
}
