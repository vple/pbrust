use std::ops::{Mul, Neg};

use derive_more::{Add, AddAssign, Sub, SubAssign};
use num::{Num, Signed};

#[derive(Debug, Copy, Clone, PartialEq)]
#[derive(Add, Sub)]
#[derive(AddAssign, SubAssign)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_vectors_are_equal() {
        let v1 = Vector3::new(1, 2, 3);
        let v2 = Vector3::new(1, 2, 3);

        assert_eq!(v1, v2);
    }

    #[test]
    fn different_vectors_are_not_equal() {
        let v1 = Vector3::new(1, 2, 3);
        let v2 = Vector3::new(-3, 0, 5);

        assert_ne!(v1, v2);
    }

    mod math {
        use super::*;

        #[test]
        fn add_vectors() {
            let v1 = Vector3::new(1, 2, 3);
            let v2 = Vector3::new(-3, 0, 5);
            let expected = Vector3::new(-2, 2, 8);

            assert_eq!(expected, v1 + v2);
        }

        #[test]
        fn add_assign_vectors() {
            let mut v1 = Vector3::new(1, 2, 3);
            let v2 = Vector3::new(-3, 0, 5);
            let expected = v1 + v2;

            v1 += v2;

            assert_eq!(expected, v1);
        }

        #[test]
        fn sub_vectors() {
            let v1 = Vector3::new(1, 2, 3);
            let v2 = Vector3::new(-3, 0, 5);
            let expected = Vector3::new(4, 2, -2);

            assert_eq!(expected, v1 - v2);
        }

        #[test]
        fn sub_assign_vectors() {
            let mut v1 = Vector3::new(1, 2, 3);
            let v2 = Vector3::new(-3, 0, 5);
            let expected = Vector3::new(4, 2, -2);

            v1 -= v2;

            assert_eq!(expected, v1);
        }
    }
}