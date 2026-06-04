use std::ops::{Add, AddAssign, Mul, Neg, Sub};
/*
 * Remaining things to implement:
 * *=
 * /=
 * scalar divide
 * dot product
 * cross multiplication
 * unit vector
 */
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}
impl Vec3 {
    pub fn x(&self) -> &f64 {
        &self.x
    }
    pub fn y(&self) -> &f64 {
        &self.y
    }
    pub fn z(&self) -> &f64 {
        &self.z
    }
    pub fn length(&self) -> f64 {
        f64::sqrt(self.length_squared())
    }
    fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
}
impl Neg for &Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}
impl Add for &Vec3 {
    type Output = Vec3;

    fn add(self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}
impl Sub for &Vec3 {
    type Output = Vec3;

    fn sub(self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}
impl Mul for &Vec3 {
    type Output = Vec3;

    fn mul(self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}
impl Mul<&f64> for &Vec3 {
    type Output = Vec3;

    fn mul(self, other: &f64) -> Vec3 {
        Vec3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}
impl Mul<&Vec3> for &f64 {
    type Output = Vec3;

    fn mul(self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: other.x * self,
            y: other.y * self,
            z: other.z * self,
        }
    }
}
impl ToString for Vec3 {
    fn to_string(&self) -> String {
        format!("{} {} {}", self.x, self.y, self.y)
    }
}

pub struct Point3(Vec3);
