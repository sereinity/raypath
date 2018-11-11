extern crate rulinalg;

use rulinalg::vector::Vector;

pub struct Ray<'a> {
    orig: &'a Vector<f64>,
    dire: Vector<f64>,
}

impl<'a> Ray<'a> {
    pub fn point_at_parameter(&self, param: f64) -> Vector<f64> {
        self.orig + &self.dire*param
    }
}

fn color(r: &Ray) -> Vector<f64> {
    let t = 0.5*r.dire[1] + 1.0;
    Vector::new(vec![1.0, 1.0, 1.0])*(1.0-t) + Vector::new(vec![0.5, 0.7, 1.0])*t
}

fn main() {
    let nx = 200;
    let ny = 100;
    println!("P3\n{} {}\n255", nx, ny);

    let llc = Vector::new(vec![-2.0, -1.0, -1.0]);
    let hori = Vector::new(vec![4.0, 0.0, 0.0]);
    let vert = Vector::new(vec![0.0, 2.0, 0.0]);
    let orig = Vector::new(vec![0.0, 0.0, 0.0]);

    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = i as f64 / nx as f64;
            let v = j as f64 / ny as f64;
            let r = Ray{orig: &orig, dire: &llc + &hori*u + &vert*v};
            let v = color(&r) * 255.99;
            println!("{} {} {}", v[0] as usize, v[1] as usize, v[2] as usize);
        }
    }
}
