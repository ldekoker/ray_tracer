use std::ops::{Add, Deref, Mul};

use crate::vec3::Vec3;

#[derive(Clone, Copy, PartialEq, Debug)]
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

pub fn write_colour(pixel_color: Colour) {
    let (r, g, b) = pixel_color.destructure();

    let rbyte: i32 = (255.999 * r) as i32;
    let gbyte: i32 = (255.999 * g) as i32;
    let bbyte: i32 = (255.999 * b) as i32;

    println!("{} {} {}", rbyte, gbyte, bbyte)
}

impl Mul<f64> for Colour {
    type Output = Colour;

    fn mul(self, rhs: f64) -> Self::Output {
        Colour(self.0 * rhs)
    }
}
impl Mul<Colour> for f64 {
    type Output = Colour;

    fn mul(self, rhs: Colour) -> Self::Output {
        Colour(self * rhs.0)
    }
}
impl Add for Colour {
    type Output = Colour;

    fn add(self, rhs: Self) -> Self::Output {
        Colour(self.0 + rhs.0)
    }
}
