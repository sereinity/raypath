fn main() {
    let nx = 200;
    let ny = 100;
    println!("P3\n{} {}\n255", nx, ny);
    for j in (0..ny).rev() {
        for i in 0..nx {
            let ir = ((i as f64 / nx as f64)*255.99) as usize;
            let ig = ((j as f64 / ny as f64)*255.99) as usize;
            let ib = (0.2*255.99) as usize;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
