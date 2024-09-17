use log::info;
use rand::distributions::Uniform;
use rand::prelude::Distribution;

use crate::{
    color,
    hittable::Hittable,
    ray::Ray,
    vec3::{cross, normalize, Vec3},
};

pub struct Camera {
    pub image_width: f32,
    pub image_height: i32,
    pub camera_origin: Vec3,
    pub pixel_delta_v: Vec3,
    pub pixel_delta_u: Vec3,
    pub pixel100_loc: Vec3,
    pub samples_per_pixel: i32,
    pub pixel_sample_scale: f32,
    pub max_ray_bounce: i32,
    pub defocus_angle: f32,
    pub defocus_disk_u: Vec3,
    pub defocus_disk_v: Vec3,
}

impl Camera {
    pub fn new(
        aspect_ratio: f32,
        image_width: f32,
        camera_origin: Vec3,
        look_at: Vec3,
        up: Vec3,
        vertical_fov: f32,
        focus_dist: f32,
        defocus_angle: f32,
    ) -> Self {
        let image_height = (image_width / aspect_ratio) as i32;

        let theta = vertical_fov.to_radians();
        let h = f32::tan(theta / 2.0);
        let viewport_height = 2.0 * h * focus_dist;
        let viewport_width = (viewport_height * (image_width / image_height as f32)) as i32;

        let camera_transform_back = normalize(camera_origin - look_at); // w
        let camera_transform_right = normalize(cross(up, camera_transform_back)); // u
        let camera_transform_up = cross(camera_transform_back, camera_transform_right); // v

        let viewport_u = camera_transform_right * viewport_width as f32;
        let viewport_v = (camera_transform_up * -1.0) * viewport_height;

        let pixel_delta_u = viewport_u / image_width;
        let pixel_delta_v = viewport_v / image_height as f32;

        let viewport_upper_left = camera_origin
            - (camera_transform_back * focus_dist)
            - viewport_u / 2.0
            - viewport_v / 2.0;
        let pixel100_loc = viewport_upper_left + ((pixel_delta_u + pixel_delta_v) * 0.5);

        let defocus_radius = focus_dist * f32::tan(f32::to_radians(defocus_angle / 2.0));
        let defocus_disk_u = camera_transform_right * defocus_radius;
        let defocus_disk_v = camera_transform_up * defocus_radius;

        let samples_per_pixel = 100;
        let pixel_sample_scale = 1.0 / samples_per_pixel as f32;
        let max_ray_bounce = 50;
        Camera {
            image_width,
            image_height,
            camera_origin,
            pixel_delta_v,
            pixel_delta_u,
            pixel100_loc,
            samples_per_pixel,
            pixel_sample_scale,
            max_ray_bounce,
            defocus_angle,
            defocus_disk_u,
            defocus_disk_v,
        }
    }

    pub fn ray_color<T>(ray: &Ray, max_ray_bounce: i32, world: &T) -> Vec3
    where
        T: Hittable,
    {
        if max_ray_bounce <= 0 {
            return Vec3::new(0.0, 0.0, 0.0);
        }

        if let Some(hit_record) = world.hit(ray, 0.001..f32::INFINITY) {
            if let Some((scattered, attenuation)) = hit_record.material.scatter(
                ray,
                hit_record.point,
                hit_record.normal,
                hit_record.front_face.unwrap_or(false),
            ) {
                return Self::ray_color(&scattered, max_ray_bounce - 1, world) * attenuation;
            }
            return Vec3::new(0.0, 0.0, 0.0);
        } else {
            let normal_dir = normalize(ray.dir);
            let a = (normal_dir.y() + 1.0) * 0.5;
            return (Vec3::new(1.0, 1.0, 1.0) * (1.0 - a)) + (Vec3::new(0.5, 0.7, 1.0) * a);
        }
    }

    pub fn defocus_disk_sample(&self) -> Vec3 {
        let v = Vec3::random_in_unit_sphere();
        return self.camera_origin + (self.defocus_disk_u * v.x()) + (self.defocus_disk_v * v.y());
    }

    pub fn sample_ray(&self, x: i32, y: i32) -> Ray {
        let mut rng = rand::thread_rng();
        let uniform = Uniform::from(0.0..1.0);
        let offset = Vec3::new(
            uniform.sample(&mut rng) - 0.5,
            uniform.sample(&mut rng) - 0.5,
            0.0,
        );
        let pixel_sample = self.pixel100_loc
            + (self.pixel_delta_u * (x as f32 + offset.x()))
            + (self.pixel_delta_v * (y as f32 + offset.y()));
        let ray_origin = if self.defocus_angle <= 0.0 {
            self.camera_origin
        } else {
            self.defocus_disk_sample()
        };
        let ray_dir = pixel_sample - ray_origin;
        let ray = Ray {
            origin: ray_origin,
            dir: ray_dir,
        };
        return ray;
    }

    pub fn render<T>(&self, world: &Vec<T>)
    where
        T: Hittable,
    {
        println!("P3");
        println!("{} {}", self.image_width, self.image_height);
        println!("255");

        for j in 0..self.image_height {
            info!("{} lines remaining", self.image_height - j);
            for i in 0..self.image_width as i32 {
                let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
                for _sample in 0..self.samples_per_pixel {
                    let ray = self.sample_ray(i, j);
                    pixel_color = pixel_color + Camera::ray_color(&ray, self.max_ray_bounce, world);
                }
                pixel_color = pixel_color * self.pixel_sample_scale;
                color::write_color(&pixel_color);
            }
        }
    }
}
