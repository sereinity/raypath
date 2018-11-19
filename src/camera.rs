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
    /// lookfrom: camera position
    /// lookat: indicates camera direction
    /// upward: the up direction of the camera
    /// vfov: vertical field of view (degrees)
    /// aspect: ration from vertical to horizontal
    pub fn new(lookfrom: Vec3, lookat: Vec3, upward: Vec3, vfov: f64, aspect: f64) -> Camera {
        let theta = vfov*f64::consts::PI/360.0;
        let half_height = f64::tan(theta/2.0);
        let half_width = aspect*half_height;
        let w = (lookfrom - lookat).normalize();
        let u = upward.cross(&w).normalize();
        let v = w.cross(&u);
        Camera {
            lower_left_corner: lookfrom - half_width*u - half_height*v - w,
            horizontal: 2.0*half_width*u,
            vertical: 2.0*half_height*v,
            origin: lookfrom,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            &self.origin,
            &self.lower_left_corner + &self.horizontal*u + &self.vertical*v - &self.origin,
            )
    }
}
