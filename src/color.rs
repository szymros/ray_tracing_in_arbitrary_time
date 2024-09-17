use std::ops::Range;

use crate::vec3::Vec3;

pub fn clamp(x: f32, interval: &Range<f32>) -> f32 {
    if x < interval.start {
        return interval.start;
    }
    if x > interval.end {
        return interval.end;
    }
    return x;
}

pub fn linear_to_gamma(linear_component: f32) -> f32 {
    if linear_component > 0.0 {
        return linear_component.sqrt();
    }
    return 0.0;
}

pub fn write_color(color: &Vec3) {
    let intensity: Range<f32> = 0.000..0.999;
    let r = linear_to_gamma(color.x());
    let g = linear_to_gamma(color.y());
    let b = linear_to_gamma(color.z());
    let rbyte = (256.0 * clamp(r, &intensity));
    let gbyte = (256.0 * clamp(g, &intensity));
    let bbyte = (256.0 * clamp(b, &intensity));
    println!("{} {} {}", rbyte, gbyte, bbyte);
}
