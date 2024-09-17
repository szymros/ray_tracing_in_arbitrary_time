mod color;
mod hittable;
mod ray;
mod vec3;
use core::f32;
mod camera;
mod material;

use camera::Camera;
use env_logger::Env;
use hittable::Sphere;
use material::{Dielectric, Lambertian, Material, Metal};
use rand::{distributions::Uniform, prelude::Distribution};
use vec3::Vec3;

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let mut rng = rand::thread_rng();
    let uniform = Uniform::from(-1.0..1.0);
    let material_ground: Box<dyn Material> = Box::new(Lambertian {
        albedo: Vec3::new(0.5, 0.5, 0.5),
    });

    let mut world = vec![Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )];
    for a in -11..11 {
        for b in -11..11 {
            let choose_material = uniform.sample(&mut rng);
            let centre = Vec3::new(
                a as f32 + 0.9 * uniform.sample(&mut rng),
                0.2,
                b as f32 + 0.9 * uniform.sample(&mut rng),
            );
            if choose_material < 0.8 {
                // diffuse
                let albedo = Vec3::random(0.0..1.0) * Vec3::random(0.0..1.0);
                let sphere_material: Box<dyn Material> = Box::new(Lambertian { albedo });
                world.push(Sphere::new(centre, 0.2, sphere_material));
            } else if choose_material < 0.95 {
                // metal
                let albedo = Vec3::random(0.0..1.0) * Vec3::random(0.0..1.0);
                let fuzz = uniform.sample(&mut rng);
                let sphere_material: Box<dyn Material> = Box::new(Metal { albedo, fuzz });
                world.push(Sphere::new(centre, 0.2, sphere_material));
            } else {
                // glass
                let sphere_material: Box<dyn Material> = Box::new(Dielectric {
                    refraction_index: 1.5,
                });
                world.push(Sphere::new(centre, 0.2, sphere_material));
            }
        }
    }
    let camera_origin = Vec3::new(13.0, 2.0, 3.0);
    let camera_look_at = Vec3::new(0.0, 0.0, 0.0);
    let camera_up = Vec3::new(0.0, 1.0, 0.0);
    let camera = Camera::new(
        16.0 / 9.0,
        1200.0,
        camera_origin,
        camera_look_at,
        camera_up,
        20.0,
        10.0,
        0.6,
    );

    camera.render(&world);
}
