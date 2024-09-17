use std::ops::{Add, Div, Mul, Range, Sub};

use rand::distributions::Uniform;
use rand::prelude::Distribution;
#[derive(Copy, Clone)]
pub struct Vec3 {
    pub cords: [f32; 3],
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 { cords: [x, y, z] }
    }
    pub fn random(rand_range: Range<f32>) -> Self {
        let mut rng = rand::thread_rng();
        let uniform = Uniform::from(rand_range);
        Vec3 {
            cords: [
                uniform.sample(&mut rng),
                uniform.sample(&mut rng),
                uniform.sample(&mut rng),
            ],
        }
    }

    pub fn random_unit() -> Self {
        loop {
            let random = Self::random(-1.0..1.0);
            let len_sqrt = random.length_squared();
            if len_sqrt > 1e-160 && len_sqrt <= 1.0 {
                return normalize(random);
            }
        }
    }
    pub fn random_in_unit_sphere() -> Self {
        loop {
            let mut rng = rand::thread_rng();
            let uniform = Uniform::from(-1.0..1.0);
            let random = Vec3 {
                cords: [uniform.sample(&mut rng), uniform.sample(&mut rng), 0.0],
            };
            if random.length_squared() < 1.0 {
                return random;
            }
        }
    }
    pub fn x(&self) -> f32 {
        self.cords[0]
    }
    pub fn y(&self) -> f32 {
        self.cords[1]
    }

    pub fn z(&self) -> f32 {
        self.cords[2]
    }

    pub fn length_squared(&self) -> f32 {
        self.x().powi(2) + self.y().powi(2) + self.z().powi(2)
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn near_zero(&self) -> bool {
        let zero: f32 = 1e-8;
        return self.cords.iter().all(|x| x.abs() < zero);
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            cords: [self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z()],
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            cords: [self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z()],
        }
    }
}
impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            cords: [self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z()],
        }
    }
}
impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Vec3 {
        Vec3 {
            cords: [self.x() * rhs, self.y() * rhs, self.z() * rhs],
        }
    }
}
impl Div for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            cords: [self.x() / rhs.x(), self.y() / rhs.y(), self.z() / rhs.z()],
        }
    }
}
impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Vec3 {
        Vec3 {
            cords: [
                self.x() * (1.0 / rhs),
                self.y() * (1.0 / rhs),
                self.z() * (1.0 / rhs),
            ],
        }
    }
}

pub fn dot(lhs: Vec3, rhs: Vec3) -> f32 {
    lhs.x() * rhs.x() + lhs.y() * rhs.y() + lhs.z() * rhs.z()
}

pub fn cross(lhs: Vec3, rhs: Vec3) -> Vec3 {
    Vec3 {
        cords: [
            lhs.y() * rhs.z() - lhs.z() * rhs.y(),
            lhs.z() * rhs.x() - lhs.x() * rhs.z(),
            lhs.x() * rhs.y() - lhs.y() * rhs.x(),
        ],
    }
}

pub fn normalize(v: Vec3) -> Vec3 {
    v / v.length()
}

pub fn random_outward(normal: Vec3) -> Vec3 {
    let random_unit = Vec3::random_unit();
    if dot(random_unit, normal) > 0.0 {
        return random_unit;
    } else {
        return random_unit * -1.0;
    }
}

pub fn reflect(v: Vec3, normal: Vec3) -> Vec3 {
    return v - normal * 2.0 * dot(v, normal);
}

pub fn refract(uv: Vec3, normal: Vec3, etai_over_etat: f32) -> Vec3 {
    // snells law
    let cos_theta = f32::min(dot(uv * -1.0, normal), 1.0);
    let out_perp = (uv + normal * cos_theta) * etai_over_etat;
    let out_parallel = normal * (-f32::sqrt(f32::abs(1.0 - out_perp.length_squared())));
    return out_perp + out_parallel;
}
