use rulinalg::vector::Vector;
use crate::ray::Ray;

pub struct Camera {
    lower_left_corner: Vector<f64>,
    horizontal: Vector<f64>,
    vertical: Vector<f64>,
    origin: Vector<f64>,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            lower_left_corner: Vector::new(vec![-2.0, -1.0, -1.0]),
            horizontal: Vector::new(vec![4.0, 0.0, 0.0]),
            vertical: Vector::new(vec![0.0, 2.0, 0.0]),
            origin: Vector::new(vec![0.0, 0.0, 0.0]),
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            &self.origin,
            &self.lower_left_corner + &self.horizontal*u + &self.vertical*v - &self.origin,
            )
    }
}
