use std::ops::Range;

use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{dot, Vec3};

pub struct HitRecord<'a> {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: Option<bool>,
    pub material: &'a dyn Material,
}

impl<'a> HitRecord<'a> {
    // the ray can come from both inside and outside of the object
    // according to that we need to set the normal and rememer which face we hit
    // we assumne that the normal that is passed here is normalized
    pub fn set_face_normal(&mut self, ray: &Ray, normal: Vec3) {
        let front_face = dot(ray.dir, normal) < 0.0;
        self.normal = if front_face { normal } else { normal * -1.0 };
        self.front_face = Some(front_face);
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, interval: Range<f32>) -> Option<HitRecord>;
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Box<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Box<dyn Material>) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, interval: Range<f32>) -> Option<HitRecord> {
        // sphere equation is for point(A,B,C) is (A-x)^2 - (B-y)^2 - (C-z)^2
        // vector from point P to centre C can be represented as C-P
        // you can rewrite the sphere equation to use the definiton of the dot product so that
        // (C-P) dot (C-P) = r^2
        // we can rewrite the P to use our ray_at definition so P = origin + t*delta
        // so C-(origin+td) dot C-(origin+td) d
        // we can rewrite this to a quadratic equation to solve for t
        // (t^2)*d*d -2td * (C-Origin) + (C-Origin) * (C-Origin) - r^2 = 0
        // now this is a matter of fiding the roots
        let oc = self.center - ray.origin;
        // dot product of a vector by itself is equal to its lenght squared
        let a = ray.dir.length_squared();
        // we can skip -2.0 now so late we dont have to do additional multiplications
        // let b = -2.0 * dot(&ray.dir, &oc);
        let half_b = dot(ray.dir, oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let delta = half_b * half_b - a * c;
        if delta < 0.0 {
            return None;
        }

        let sqrt_d = delta.sqrt();

        let mut root = (half_b - sqrt_d) / a;
        if !interval.contains(&root) {
            root = (half_b + sqrt_d) / a;
            if !interval.contains(&root) {
                return None;
            }
        }
        let hit_point = ray.at(root);
        let normal = (hit_point - self.center) / self.radius;
        let mut hit_record = HitRecord {
            point: hit_point,
            t: root,
            normal,
            front_face: None,
            material: &*self.material,
        };
        hit_record.set_face_normal(ray, normal);

        return Some(hit_record);
    }
}

impl<T> Hittable for Vec<T>
where
    T: Hittable,
{
    fn hit(&self, ray: &Ray, interval: Range<f32>) -> Option<HitRecord> {
        let mut closest_t = interval.end;
        let mut hit_rec = None;
        for item in self {
            if let Some(temp_rec) = item.hit(ray, interval.start..closest_t) {
                closest_t = temp_rec.t;
                hit_rec = Some(temp_rec);
            }
        }
        return hit_rec;
    }
}
