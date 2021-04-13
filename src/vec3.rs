use rand::random;
use std::f64::consts::PI;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub type Color = Vec3;

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn random_unit() -> Self {
        // a & b are random angles
        let a = random::<f64>() * 2.0 * PI;
        let b = random::<f64>() * 2.0 * PI;

        Self {
            x: a.cos() * b.cos(),
            y: a.sin(),
            z: a.cos() * b.sin(),
        }
    }

    pub fn random_in_unit_sphere() -> Self {
        let n = random::<f64>();
        Self::random_unit() * n
    }

    pub fn random_in_hemisphere(normal: Self) -> Self {
        let in_unit = Self::random_in_unit_sphere();
        if in_unit.dot(normal) > 0.0 {
            in_unit
        } else {
            -in_unit
        }
    }

    pub fn len(&self) -> f64 {
        let Self { x, y, z } = self;
        (x * x + y * y + z * z).sqrt()
    }

    pub fn normalize(self) -> Self {
        self / self.len()
    }

    pub fn dot(&self, other: Self) -> f64 {
        let Self {
            x: x1,
            y: y1,
            z: z1,
        } = self;
        let Self {
            x: x2,
            y: y2,
            z: z2,
        } = other;
        x1 * x2 + y1 * y2 + z1 * z2
    }

    pub fn cross(&self, other: Self) -> Self {
        let Self {
            x: x1,
            y: y1,
            z: z1,
        } = self;
        let Self {
            x: x2,
            y: y2,
            z: z2,
        } = other;

        Self {
            x: y1 * z2 - z1 * y2,
            y: z1 * x2 - x1 * z2,
            z: x1 * y2 - y1 * x2,
        }
    }

    pub fn reflect(self, normal: Self) -> Self {
        self - normal * 2.0 * self.dot(normal)
    }

    pub fn any_near_zero(&self) -> bool {
        let s = 1e-8;
        (self.x.abs() < s) && (self.y.abs() < s) && (self.z.abs() < s)
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Self> for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        self * -1.0
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Self::Output {
        Vec3 {
            x: v.x * self,
            y: v.y * self,
            z: v.z * self,
        }
    }
}
