use std::ops::Deref;

use crate::vec3::Vec3;

pub struct Colour(Vec3);

impl Colour {
    pub fn new(x: f64, y: f64, z: f64) -> Colour {
        Colour(Vec3::new(x, y, z))
    }
}

impl Deref for Colour {
    type Target = Vec3;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn write_colour(pixel_color: &Colour) {
    let (r, g, b) = pixel_color.destructure();

    let rbyte: i32 = (255.999 * *r) as i32;
    let gbyte: i32 = (255.999 * *g) as i32;
    let bbyte: i32 = (255.999 * *b) as i32;

    println!("{} {} {}", rbyte, gbyte, bbyte)
}
