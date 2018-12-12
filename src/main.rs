use rand::prelude::*;

use raytracer::Vec3;
use raytracer::ray::{Ray, HitRec};
use raytracer::camera::Camera;
use raytracer::object::Object;
use raytracer::sphere::Sphere;
use raytracer::material::Material;

fn hit<'a>(obj_list: &'a Vec<Box<dyn Object>>, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRec<'a>> {
    let mut hit_rec: Option<HitRec> = None;
    let mut closest_so_far = t_max;
    for object in obj_list {
        if let Some(temp_hit_rec) = object.hit(ray, t_min, closest_so_far) {
            closest_so_far = temp_hit_rec.t;
            hit_rec = Some(temp_hit_rec);
        }
    }
    hit_rec
}

fn color(r: &Ray, world: &Vec<Box<dyn Object>>, depth: usize) -> Vec3 {
    match hit(world, r, 0.0001, std::f64::INFINITY) {
        Some(hit_rec) => {
            if depth < 50 {
                if let Some((attenuation, scattered)) = hit_rec.material.scatter(r, &hit_rec) {
                    color(&scattered, &world, depth+1).component_mul(&attenuation)
                } else {
                    Vec3::zeros()
                }
            } else {
                Vec3::zeros()
            }
        }
        None => {
            let t = 0.5*r.dire.normalize()[1] + 1.0;
            Vec3::from_element(1.0)*(1.0-t) + Vec3::new(0.5, 0.7, 1.0)*t
        }
    }
}

fn main() {
    let nx = 200;
    let ny = 100;
    let ns = 100;

    let world = random_scene();

    let lookfrom = Vec3::new(11.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let dist_to_focus = (lookfrom-lookat).norm();
    let cam = Camera::new(lookfrom, lookat, Vec3::new(0.0, 1.0, 0.0), 45.0, nx as f64/ny as f64, 0.1, dist_to_focus);
    let mut rng = thread_rng();

    println!("P3\n{} {}\n255", nx, ny);
    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..ns {
                let u = (i as f64 + rng.gen::<f64>()) / nx as f64;
                let v = (j as f64 + rng.gen::<f64>()) / ny as f64;
                let r = cam.get_ray(u, v);
                col += color(&r, &world, 0);
            }
            let col = col.map(|x| ((x/ns as f64).sqrt()*255.99) as usize);
            println!("{} {} {}", col[0], col[1], col[2]);
        }
    }
}

fn random_scene() -> Vec<Box<dyn Object>> {
    let mut rng = thread_rng();

    let mut world: Vec<Box<dyn Object>> = Vec::with_capacity(500);
    world.push(Box::new(Sphere {
        center: Vec3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Material::Lambertian(Vec3::new(0.5, 0.5, 0.5)),
    }));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen::<f64>();
            let center = Vec3::new(a as f64 + 0.9 + rng.gen::<f64>(), 0.2, b as f64 + rng.gen::<f64>());
            if (center - Vec3::new(4.0, 0.2, 0.0)).norm() > 0.9 {
                if choose_mat < 0.8 { // difuse
                    world.push(Box::new(Sphere{
                        center: center,
                        radius: 0.2,
                        material: Material::Lambertian(0.5*Vec3::from_fn(|_, _| rng.gen::<f64>()*rng.gen::<f64>())),
                    }));
                } else if choose_mat < 0.95 {
                    world.push(Box::new(Sphere{
                        center: center,
                        radius: 0.2,
                        material: Material::Metal(0.5*Vec3::from_fn(|_, _| 1.0 + rng.gen::<f64>()), rng.gen::<f64>()),
                    }));
                } else {
                    world.push(Box::new(Sphere{
                        center: center,
                        radius: 0.2,
                        material: Material::Dielectric(1.5),
                    }));
                }
            }
        }
    }

    world.push(Box::new(Sphere{
        center: Vec3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        material: Material::Dielectric(1.5),
    }));
    world.push(Box::new(Sphere{
        center: Vec3::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: Material::Lambertian(Vec3::new(0.4, 0.2, 0.1)),
    }));
    world.push(Box::new(Sphere{
        center: Vec3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material: Material::Metal(Vec3::new(0.7, 0.6, 0.5), 0.0),
    }));

    world
}
