use std::ops::Mul;

pub fn first_integral<T: Mul<f64>>(t: &f64, dx_dt: T) -> T::Output {
    dx_dt * *t
}

pub fn second_integral<T: Mul<f64>>(t: &f64, d2x_dt2: T) -> T::Output {
    d2x_dt2 * (0.5 * *t * *t)
}

#[cfg(test)]
mod test {
    use approx::assert_relative_eq;

    use crate::math::{first_integral, second_integral};
    use crate::point::Point;

    #[test]
    fn first_integral_works_with_floats() {
        let t = 0.5;
        let dx_dt = 4.;

        let x = first_integral(&t, dx_dt);

        assert_relative_eq!(x, 2.);
    }

    #[test]
    fn first_integral_works_with_points() {
        let t = 0.5;
        let dx_dt = Point::new3(1., 2., 3.);

        let x = first_integral(&t, &dx_dt);

        assert_relative_eq!(&x, &Point::new3(0.5, 1., 1.5));
    }

    #[test]
    fn second_integral_works_with_floats() {
        let t = 0.5;
        let d2x_dt2 = 4.;

        let x = second_integral(&t, d2x_dt2);

        assert_relative_eq!(x, 0.5);
    }

    #[test]
    fn second_integral_works_with_points() {
        let t = 0.5;
        let d2x_dt2 = Point::new3(8., 16., 24.);

        let x = second_integral(&t, &d2x_dt2);

        assert_relative_eq!(&x, &Point::new3(1., 2., 3.));
    }
}
