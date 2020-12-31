use std::ops::{AddAssign, Mul, Neg, SubAssign};

use derive_more::{Add, Sub};
use num::{Num, Signed};

#[derive(Debug, Copy, Clone)]
#[derive(Add, Sub)]
pub struct Vector3<T: Num> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Num> Vector3<T> {
    pub fn new(x: T, y: T, z: T) -> Vector3<T> {
        Vector3 { x, y, z }
    }
}

impl<T: Signed> Vector3<T> {
    pub fn abs(&self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs(),
        }
    }
}

impl<T> Neg for Vector3<T> where T: Neg<Output=T> + Num {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl<T> AddAssign for Vector3<T> where T: AddAssign + Num {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl<T> SubAssign for Vector3<T> where T: SubAssign + Num {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl<T> Mul<T> for Vector3<T> where T: Mul<Output=T> + Num + Copy {
    type Output = Vector3<T>;

    fn mul(self, other: T) -> Self::Output {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}