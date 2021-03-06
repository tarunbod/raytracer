// Math Code
use std::ops;
use rand::prelude::*;

#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Vec3 {
    pub const ORIGIN: Vec3 = Vec3 { x: 0.0, y: 0.0, z: 0.0 };

    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 {
            x, y, z
        }
    }

    pub fn origin() -> Vec3 {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
        u.x * v.x + u.y * v.y + u.z * v.z
    }

    pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
        Vec3 {
            x: u.y * v.z - u.z * v.y,
            y: u.z * v.x - u.x * v.z,
            z: u.x * v.y - u.y * v.x
        }
    }

    pub fn random(min: f64, max: f64) -> Vec3 {
        let mut rng = rand::thread_rng();
        let x = rng.gen::<f64>() * (max - min) + min;
        let y = rng.gen::<f64>() * (max - min) + min;
        let z = rng.gen::<f64>() * (max - min) + min;
        Vec3 { x, y, z }
    }

    pub fn rgb(r: u8, g: u8, b: u8) -> Vec3 {
        Self::new(r as f64 / 255.0, g as f64 / 255.0, b as f64 / 255.0)
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        Self::dot(self, self)
    }

    pub fn unit(self) -> Vec3 {
        let l  = self.length();
        self / l
    }

    pub fn gamma_correct(self, value: f64) -> Color {
        Color {
            x: self.x.powf(1.0 / value),
            y: self.y.powf(1.0 / value),
            z: self.z.powf(1.0 / value),
        }
    }
}

pub fn random_vec_in_unit_sphere() -> Vec3 {
    loop {
        let v = Vec3::random(-1.0, 1.0);
        if v.length_squared() < 1.0 {
            return v;
        }
    }
}

pub fn random_unit_vec() -> Vec3 {
    random_vec_in_unit_sphere().unit()
}

pub fn random_hemisphere_vec(n: Vec3) -> Vec3 {
    let v = random_unit_vec();
    if Vec3::dot(&v, &n) > 0.0 {
        v
    } else {
        -1.0 * v
    }
}

impl ops::Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl ops::Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}

impl ops::SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl ops::Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z
        }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Vec3 {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs
        }
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z
        }
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Vec3 {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs
        }
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

pub type Point = Vec3;
pub type Color = Vec3;

#[derive(Debug)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vec3
}
impl Ray {
    pub fn new(origin: Point, direction: Vec3) -> Ray {
        Ray {
            origin, direction
        }
    }
    pub fn at(&self, t: f64) -> Point {
        self.origin + t * self.direction
    }
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

pub fn in_range(x: f64, min: f64, max: f64) -> bool {
    min <= x && x <= max
}