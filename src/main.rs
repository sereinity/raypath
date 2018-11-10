extern crate rulinalg;

use rulinalg::vector::Vector;

fn main() {
    let nx = 200;
    let ny = 100;
    println!("P3\n{} {}\n255", nx, ny);
    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut v: Vec<f64> = Vec::with_capacity(3);
            v.push(i as f64 / nx as f64);
            v.push(j as f64 / ny as f64);
            v.push(0.2);
            let v = Vector::new(v);
            let iv = v * 255.99;
            println!("{} {} {}", iv[0] as usize, iv[1] as usize, iv[2] as usize);
        }
    }
}
