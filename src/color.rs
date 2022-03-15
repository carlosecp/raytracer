use crate::tuple::Tuple;
use std::ops::{Add, Mul, Sub};

// TODO: I had to make the Tuple fields public for this to work.
pub const DEFAULT_COLOR: Color = Color(Tuple(0., 0., 0.));

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Color(Tuple);

impl Color {
    pub fn new(red: f64, green: f64, blue: f64) -> Self {
        Color(Tuple::new(red, green, blue))
    }

    pub fn red(&self) -> f64 {
        self.0.values()[0]
    }

    pub fn green(&self) -> f64 {
        self.0.values()[1]
    }

    pub fn blue(&self) -> f64 {
        self.0.values()[2]
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Color(self.0 + other.0)
    }
}

impl Mul for Color {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Color(self.0 * other.0)
    }
}

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, factor: f64) -> Self::Output {
        Color(self.0 * factor)
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Color(self.0 - other.0)
    }
}

#[cfg(test)]
mod create {
    use super::*;

    #[test]
    fn create_color() {
        let c = Color::new(-0.5, 0.4, 1.7);

        assert_eq!(c.red(), -0.5);
        assert_eq!(c.green(), 0.4);
        assert_eq!(c.blue(), 1.7);
    }
}

#[cfg(test)]
mod ops {
    use super::*;

    #[test]
    fn adding_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);

        assert_eq!(c1 + c2, Color::new(1.6, 0.7, 1.));
    }

    #[test]
    fn subtracting_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);

        assert_eq!(c1 - c2, Color::new(0.2, 0.5, 0.5));
    }

    #[test]
    fn multiplying_color_by_scalar() {
        let c = Color::new(0.2, 0.3, 0.4);

        assert_eq!(c * 2., Color::new(0.4, 0.6, 0.8));
    }

    #[test]
    fn multiplying_color_by_color() {
        let c1 = Color::new(1., 0.2, 0.4);
        let c2 = Color::new(0.9, 1., 0.1);

        assert_eq!(c1 * c2, Color::new(0.9, 0.2, 0.04));
    }
}
