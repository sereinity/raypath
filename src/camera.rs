use std::f64;
use crate::Vec3;
use crate::ray::Ray;

pub struct Camera {
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3,
}

impl Camera {
    /// Creates a camera
    /// vfov: vertical field of view (degrees)
    /// aspect: ration from vertical to horizontal
    pub fn new(vfov: f64, aspect: f64) -> Camera {
        let theta = vfov*f64::consts::PI/360.0;
        let half_height = f64::tan(theta/2.0);
        let half_width = aspect*half_height;
        Camera {
            lower_left_corner: Vec3::new(-half_width, -half_height, -1.0),
            horizontal: Vec3::new(2.0*half_width, 0.0, 0.0),
            vertical: Vec3::new(0.0, 2.0*half_height, 0.0),
            origin: Vec3::new(0.0, 0.0, 0.0),
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            &self.origin,
            &self.lower_left_corner + &self.horizontal*u + &self.vertical*v - &self.origin,
            )
    }
}
