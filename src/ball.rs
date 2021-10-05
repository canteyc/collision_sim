use crate::point::Point;
use crate::math::{first_integral, second_integral};

#[derive(Debug, PartialEq)]
struct Ball {
    position: Point,
    velocity: Point,
    radius: f64,
}

impl Ball {
    pub fn new() -> Ball {
        Ball {
            position: Point::new(),
            velocity: Point::new(),
            radius: 0.,
        }
    }

    pub fn with_radius(radius: f64) -> Ball {
        Ball {
            radius,
            ..Ball::new()
        }
    }

    pub fn update(&self, dt: &f64, acceleration: &Point) -> Ball {
        Ball {
            position: &(&self.position + &first_integral(dt, &self.velocity)) + &second_integral(dt, acceleration),
            velocity: &self.velocity + &first_integral(dt, acceleration),
            ..*self
        }
    }
}


#[cfg(test)]
mod test {
    use crate::ball::Ball;
    use crate::point::Point;
    use approx::assert_relative_eq;

    fn ball1() -> Ball {
        Ball::with_radius(1.)
    }

    #[test]
    fn make_ball() {
        let ball = Ball::new();

        let expected = Ball {
            position: Point::new(),
            velocity: Point::new(),
            radius: 0.,
        };

        assert_eq!(ball, expected);
    }

    #[test]
    fn unmoving_ball_does_not_move() {
        let dt = 1.;
        let acceleration = Point::new();
        let ball = ball1();

        let next_ball = ball.update(&dt, &acceleration);

        assert_eq!(ball, next_ball);
    }
    
    #[test]
    fn drop_ball_for_ten_seconds() {
        let dt = 10.;
        let acceleration = Point::new3(0., 0., -9.81);
        let ball = ball1();
        
        let next_ball = ball.update(&dt, &acceleration);
        let expected = Ball {
            position: Point::new3(0., 0., -490.5),
            velocity: Point::new3(0., 0., -98.1),
            radius: 1.0
        };

        assert_relative_eq!(&next_ball.position, &expected.position);
        assert_relative_eq!(&next_ball.velocity, &expected.velocity);
        assert_relative_eq!(next_ball.radius, expected.radius);
    }

    #[test]
    fn parabolic_path() {
        let dt = 10.;
        let acceleration = Point::new3(0., 0., -9.81);
        let ball = Ball {
            position: Point::new(),
            velocity: Point::new3(0., 0., 49.05),  // half the speed from the 10 second drop test
            radius: 1.0
        };

        let next_ball = ball.update(&dt, &acceleration);

        assert_relative_eq!(&next_ball.position, &ball.position);
        assert_relative_eq!(&next_ball.velocity, &(&ball.velocity * -1.));
        assert_relative_eq!(next_ball.radius, ball.radius);
    }
}