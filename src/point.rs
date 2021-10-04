use std::ops::{Add, Mul, Sub};

#[derive(Debug, PartialEq)]
struct Point {
    x: f64,
    y: f64,
    z: f64,
}

impl Point {
    fn new() -> Point {
        Point::new3(0., 0., 0.)
    }

    fn new2(x: f64, y: f64) -> Point {
        Point::new3(x, y, 0.)
    }

    fn new3(x: f64, y: f64, z: f64) -> Point {
        Point {x, y, z}
    }

    fn mag(&self) -> f64 {
        f64::sqrt(self.x * self.x + self.y * self.y + self.z * self.z)
    }

    fn unit(&self) -> Point {
        self * (1. / self.mag())
    }
}

impl Add for &Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for &Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul<f64> for &Point {
    type Output = Point;

    fn mul(self, rhs: f64) -> Self::Output {
        Point {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul for &Point {
    type Output = f64;

    fn mul(self, rhs: Self) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

#[cfg(test)]
mod test {
    use crate::point::*;
    use approx::assert_relative_eq;

    fn point00() -> Point {
        Point::new()
    }

    fn point12() -> Point {
        Point::new2(1., 2.)
    }

    fn point34() -> Point {
        Point::new2(3., 4.)
    }

    #[test]
    fn create_point_starts_with_zeros() {
        let pt = point00();

        assert_eq!(pt.x, 0.);
        assert_eq!(pt.y, 0.);
        assert_eq!(pt.z, 0.);
    }

    #[test]
    fn create_2d_point_sets_z_to_sero() {
        let pt = point12();

        assert_eq!(pt.x, 1.);
        assert_eq!(pt.y, 2.);
        assert_eq!(pt.z, 0.);
    }

    #[test]
    fn add_points_adds_points() {
        let pt1 = point12();
        let pt2 = point34();

        let pt3 = &pt1 + &pt2;

        assert_eq!(pt3, Point::new2(4., 6.));
    }

    #[test]
    fn points_with_same_values_are_equal() {
        let pt1 = point12();
        let pt2 = point12();

        assert_eq!(pt1.x, pt2.x);
        assert_eq!(pt1.y, pt2.y);
        assert_eq!(pt1.z, pt2.z);
        assert_eq!(pt1, pt2);
    }

    #[test]
    fn sub_points_subtracts_points() {
        let pt1 = point12();
        let pt2 = point34();

        let pt3 = &pt1 - &pt2;

        assert_eq!(pt3, Point::new2(-2., -2.));
    }

    #[test]
    fn points_scale_when_multiplied_by_floats() {
        let pt1 = point12();
        let factor = 3.;

        let pt2 = &pt1 * factor;

        assert_eq!(pt2, Point::new2(3., 6.));
    }

    #[test]
    fn scaling_points_only_needs_a_reference() {
        let pt1 = point12();
        let factor = 3.;

        let _ = &pt1 * factor;

        assert_eq!(pt1, point12());
    }

    #[test]
    fn multiply_points_performs_dot_product() {
        let pt1 = point12();
        let pt2 = point34();

        let product = &pt1 * &pt2;

        assert_eq!(product, 11.);
    }

    #[test]
    fn magnitude_computes_l2_norm() {
        let pt1 = point34();

        let mag = pt1.mag();

        assert_eq!(mag, 5.);
    }

    #[test]
    fn mag_works_in_3d() {
        let pt1 = Point::new3(1., 2., 3.);

        let mag = pt1.mag();

        assert_eq!(mag, f64::sqrt(14.));
    }

    #[test]
    fn unit_vector_has_length_one() {
        let pt1 = point12();

        let pt1_unit = pt1.unit();

        assert_relative_eq!(pt1_unit.mag(), 1.);

        assert_eq!(pt1_unit, Point::new2(1. / f64::sqrt(5.), 2. / f64::sqrt(5.)));
    }

    #[test]
    fn unit_vectors_can_be_negative() {
        let pt1 = Point::new3(-3., 4., -5.);

        let pt1_unit = pt1.unit();

        assert_eq!(pt1_unit, Point::new3(-3. / f64::sqrt(50.), 4. / f64::sqrt(50.), -5. / f64::sqrt(50.)));
    }
}