use clap::{crate_version, load_yaml, App};

use rand::prelude::*;

use raytracing::camera::Camera;
use raytracing::material::Material;
use raytracing::object::Object;
use raytracing::ray::render;
use raytracing::sphere::Sphere;
use raytracing::Vec3;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).version(crate_version!()).get_matches();
    let (nx, ny, ns) = match matches.value_of("quality").unwrap() {
        "medium" => (400, 200, 200),
        "high" => (1000, 500, 1000),
        "HD" => (1920, 1080, 1000),
        "4K" => (4096, 2160, 1000),
        "low" => (200, 100, 100), // also the default
        _ => (200, 100, 100),
    };

    let world = random_scene();

    let lookfrom = Vec3::new(11.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let dist_to_focus = (lookfrom - lookat).norm();
    let cam = Camera::new(
        lookfrom,
        lookat,
        Vec3::new(0.0, 1.0, 0.0),
        45.0,
        nx as f64 / ny as f64,
        0.1,
        dist_to_focus,
    );
    let pixs = render(world, cam, nx, ny, ns);

    image::save_buffer(
        matches.value_of("output").unwrap(),
        &pixs,
        nx as u32,
        ny as u32,
        image::ColorType::Rgba8,
    )
    .expect("Can't save the image");
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
            let center = Vec3::new(
                a as f64 + 0.9 + rng.gen::<f64>(),
                0.2,
                b as f64 + rng.gen::<f64>(),
            );
            if (center - Vec3::new(4.0, 0.2, 0.0)).norm() > 0.9 {
                if choose_mat < 0.8 {
                    // difuse
                    world.push(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Material::Lambertian(
                            0.5 * Vec3::from_fn(|_, _| rng.gen::<f64>() * rng.gen::<f64>()),
                        ),
                    }));
                } else if choose_mat < 0.95 {
                    world.push(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Material::Metal(
                            0.5 * Vec3::from_fn(|_, _| 1.0 + rng.gen::<f64>()),
                            rng.gen::<f64>(),
                        ),
                    }));
                } else {
                    world.push(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Material::Dielectric(1.5),
                    }));
                }
            }
        }
    }

    world.push(Box::new(Sphere {
        center: Vec3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        material: Material::Dielectric(1.5),
    }));
    world.push(Box::new(Sphere {
        center: Vec3::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: Material::Lambertian(Vec3::new(0.4, 0.2, 0.1)),
    }));
    world.push(Box::new(Sphere {
        center: Vec3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material: Material::Metal(Vec3::new(0.7, 0.6, 0.5), 0.0),
    }));

    world
}
