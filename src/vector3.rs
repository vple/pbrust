use std::ops::Neg;

use derive_more::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use num::{Num, Signed};

#[derive(Debug, Copy, Clone, PartialEq)]
#[derive(Add, Sub, Mul, Div)]
#[derive(AddAssign, SubAssign, MulAssign, DivAssign)]
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

        #[test]
        fn mul_vector_and_scalar() {
            let v = Vector3::new(1, 2, 3);
            let expected = Vector3::new(-2, -4, -6);

            assert_eq!(expected, v * -2);

            let v = Vector3::new(1.0, 2.0, 3.0);
            let expected = Vector3::new(-2.5, -5.0, -7.5);

            assert_eq!(expected, v * -2.5);
        }

        #[test]
        fn mul_assign_vector_and_scalar() {
            let mut v = Vector3::new(1, 2, 3);
            let expected = v * -2;

            v *= -2;

            assert_eq!(expected, v);

            let mut v = Vector3::new(1.0, 2.0, 3.0);
            let expected = v * -2.5;

            v *= -2.5;

            assert_eq!(expected, v);
        }

        #[test]
        fn div_vector_and_scalar() {
            let v = Vector3::new(-2, -4, -6);
            let expected = Vector3::new(1, 2, 3);

            assert_eq!(expected, v / -2);

            let v = Vector3::new(-2.5, -5.0, -7.5);
            let expected = Vector3::new(1.0, 2.0, 3.0);

            assert_eq!(expected, v / -2.5);
        }

        #[test]
        fn div_assign_vector_and_scalar() {
            let mut v = Vector3::new(-2, -4, -6);
            let expected = v / -2;

            v /= -2;

            assert_eq!(expected, v);

            let mut v = Vector3::new(-2.5, -5.0, -7.5);
            let expected = v / -2.5;

            v /= -2.5;

            assert_eq!(expected, v);
        }
    }
}