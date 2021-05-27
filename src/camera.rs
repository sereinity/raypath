use rand::prelude::*;
use std::f64;

use crate::ray::Ray;
use crate::Vec3;

pub struct Camera {
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3,
    lens_radiaus: f64,
    u: Vec3,
    v: Vec3,
}

impl Camera {
    /// Creates a camera
    /// lookfrom: camera position
    /// lookat: indicates camera direction
    /// upward: the up direction of the camera
    /// vfov: vertical field of view (degrees)
    /// aspect: ration from vertical to horizontal
    /// apperture: the apperture
    /// focus_dist: how far is the focus setted
    pub fn new(
        lookfrom: Vec3,
        lookat: Vec3,
        upward: Vec3,
        vfov: f64,
        aspect: f64,
        apperture: f64,
        focus_dist: f64,
    ) -> Camera {
        let theta = vfov * f64::consts::PI / 360.0;
        let half_height = f64::tan(theta / 2.0);
        let half_width = aspect * half_height;
        let w = (lookfrom - lookat).normalize();
        let u = upward.cross(&w).normalize();
        let v = w.cross(&u);
        Camera {
            lower_left_corner: lookfrom - (half_width * u + half_height * v + w) * focus_dist,
            horizontal: 2.0 * half_width * focus_dist * u,
            vertical: 2.0 * half_height * focus_dist * v,
            origin: lookfrom,
            lens_radiaus: apperture / 2.0,
            u,
            v,
        }
    }

    pub(crate) fn get_ray(&self, u: f64, v: f64) -> Ray {
        let rd = self.lens_radiaus * random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;
        let origin = self.origin + offset;
        Ray::new(
            origin,
            self.lower_left_corner + self.horizontal * u + self.vertical * v - origin,
        )
    }
}

fn random_in_unit_disk() -> Vec3 {
    let mut rng = thread_rng();
    loop {
        // Random taken from a square of 2x2x0 centered on 1x1x1
        let mut p = Vec3::new(rng.gen::<f64>(), rng.gen::<f64>(), 0.0) * 2.0;
        p -= Vec3::new(1.0, 1.0, 0.0);
        if p.dot(&p) < 1.0 {
            // Is it in a disk?
            return p;
        }
    }
}
