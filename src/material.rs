use rand::{distributions::Uniform, prelude::Distribution};

use crate::{
    ray::Ray,
    vec3::{dot, normalize, reflect, refract, Vec3},
};

pub trait Material {
    fn scatter(
        &self,
        ray: &Ray,
        point: Vec3,
        normal: Vec3,
        front_face: bool,
    ) -> Option<(Ray, Vec3)>;
}

pub struct Lambertian {
    pub albedo: Vec3,
}

impl Material for Lambertian {
    fn scatter(
        &self,
        ray: &Ray,
        point: Vec3,
        normal: Vec3,
        front_face: bool,
    ) -> Option<(Ray, Vec3)> {
        let mut scatter_direction = normal + Vec3::random_unit();
        if scatter_direction.near_zero() {
            scatter_direction = normal;
        }
        let scattered = Ray {
            origin: point,
            dir: scatter_direction,
        };
        return Some((scattered, self.albedo));
    }
}

pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f32,
}

impl Material for Metal {
    fn scatter(
        &self,
        ray: &Ray,
        point: Vec3,
        normal: Vec3,
        front_face: bool,
    ) -> Option<(Ray, Vec3)> {
        let reflected: Vec3 =
            normalize(reflect(ray.dir, normal)) + (Vec3::random_unit() * self.fuzz);
        let scattered = Ray {
            origin: point,
            dir: reflected,
        };
        if dot(scattered.dir, normal) > 0.0 {
            return Some((scattered, self.albedo));
        }
        return None;
    }
}

pub struct Dielectric {
    pub(crate) refraction_index: f32,
}

impl Dielectric {
    fn reflactance(&self, cos: f32, refraction_index: f32) -> f32 {
        // Shlicks approximation for reflactance
        let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        r0 = f32::powi(r0, 2);
        return r0 + (1.0 - r0) * f32::powi(1.0 - cos, 5);
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        ray: &Ray,
        point: Vec3,
        normal: Vec3,
        front_face: bool,
    ) -> Option<(Ray, Vec3)> {
        // snells law + consider total internal reflection
        let attenuation = Vec3::new(1.0, 1.0, 1.0);
        let refraction_index = if front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };
        let unit_direction = normalize(ray.dir);
        let cos_theta = f32::min(dot(unit_direction * -1.0, normal), 1.0);
        let sin_theta = f32::sqrt(1.0 - cos_theta.powi(2));
        let mut rng = rand::thread_rng();
        let uniform = Uniform::from(0.0..1.0);
        let direction = if refraction_index * sin_theta > 1.0
            || self.reflactance(cos_theta, refraction_index) > uniform.sample(&mut rng)
        {
            reflect(unit_direction, normal)
        } else {
            refract(unit_direction, normal, refraction_index)
        };
        let scattered = Ray {
            origin: point,
            dir: direction,
        };
        return Some((scattered, attenuation));
    }
}
