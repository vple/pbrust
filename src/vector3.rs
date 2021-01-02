use derive_more::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use num::{abs, Num, Signed, ToPrimitive};

#[derive(Debug, Copy, Clone, PartialEq)]
#[derive(Neg)]
#[derive(Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign)]
pub struct Vector3<T: Num> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Num> Vector3<T> {
    /// Creates a new vector.
    pub fn new(x: T, y: T, z: T) -> Vector3<T> {
        Vector3 { x, y, z }
    }

    /// Computes the dot product of this vector with the given vector.
    ///
    /// The dot product is calculated by multiplying the corresponding vector fields, then summing
    /// those products.
    ///
    /// # Examples
    ///
    /// ```
    /// use pbrust::vector3::Vector3;
    ///
    /// let v1 = Vector3::new(1, 2, 3);
    /// let v2 = Vector3::new(2, 4, 6);
    /// let expected = 1*2 + 2*4 + 3*6;
    ///
    /// assert_eq!(expected, v1.dot(v2));
    /// ```
    pub fn dot(self, other: Vector3<T>) -> T {
        return self.x * other.x + self.y * other.y + self.z * other.z;
    }
}

impl<T: Num + Copy + ToPrimitive> Vector3<T> {
    /// Computes the cross product of this vector with the given vector.
    pub fn cross(self, other: Vector3<T>) -> Self {
        return Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        };
    }

    // Should this actually return T?
    /// Computes the squared length of this vector.
    pub fn length_squared(self) -> f64 {
        let length_squared = self.x * self.x + self.y * self.y + self.z * self.z;
        return length_squared.to_f64().expect("Failed to convert to f64!");
    }

    /// Computes the length of this vector.
    pub fn length(self) -> f64 {
        return self.length_squared().sqrt();
    }
}

impl<T: Signed> Vector3<T> {
    /// Computes the absolute value of this vector.
    pub fn abs(&self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs(),
        }
    }

    /// Computes the absolute value of the dot product of this and another vector.
    ///
    /// # Example
    ///
    /// ```
    /// use pbrust::vector3::Vector3;
    ///
    /// let v1 = Vector3::new(1, 2, 3);
    /// let v2 = Vector3::new(-1, -2, -3);
    ///
    /// // absdot is a convenience method:
    /// let absdot1 = v1.abs_dot(v2);
    ///
    /// // It's equivalent to:
    /// let absdot2 = (v1.dot(v2) as i32).abs();
    ///
    /// assert_eq!(absdot1, absdot2);
    /// ```
    pub fn abs_dot(self, other: Vector3<T>) -> T {
        return abs(self.dot(other));
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
        fn neg() {
            let v = Vector3::new(-1, 2, -3);
            let expected = Vector3::new(1, -2, 3);

            assert_eq!(expected, -v);
        }

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

        #[test]
        fn dot_product() {
            let v1 = Vector3::new(1, -2, -3);
            let v2 = Vector3::new(2, 4, -6);
            let expected = 1 * 2 + -2 * 4 + -3 * -6;

            assert_eq!(expected, v1.dot(v2));
        }

        #[test]
        fn cross_product() {
            let v1 = Vector3::new(1, -2, -3);
            let v2 = Vector3::new(2, 4, -6);
            let expected = Vector3::new(-2 * -6 - -3 * 4, -3 * 2 - 1 * -6, 1 * 4 - -2 * 2);

            assert_eq!(expected, v1.cross(v2));
        }

        #[test]
        fn length_squared() {
            let v = Vector3::new(1, -2, -3);
            let expected = (1 * 1 + -2 * -2 + -3 * -3) as f64;

            assert_eq!(expected, v.length_squared());

            let v = Vector3::new(1.0, -2.0, -3.0);
            let expected = (1 * 1 + -2 * -2 + -3 * -3) as f64;

            assert_eq!(expected, v.length_squared());
        }

        #[test]
        fn length() {
            let v = Vector3::new(1, -2, -3);
            let expected = ((1 * 1 + -2 * -2 + -3 * -3) as f64).sqrt();

            assert_eq!(expected, v.length());

            let v = Vector3::new(1.0, -2.0, -3.0);
            let expected = ((1 * 1 + -2 * -2 + -3 * -3) as f64).sqrt();

            assert_eq!(expected, v.length());
        }
    }
}