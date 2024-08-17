use crate::{interval::Interval, vec3::Vec3};

pub type Color = Vec3;

pub fn write_color(pixel_color: &Color) {
    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.z();
    let intensity = Interval {
        max: 0.999,
        min: 0.000,
    };
    let rbyte = (256. * intensity.clamp(r)) as i32;
    let gbyte = (256. * intensity.clamp(g)) as i32;
    let bbyte = (256. * intensity.clamp(b)) as i32;

    println!("{} {} {}", rbyte, gbyte, bbyte);
}
