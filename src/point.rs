use std::ops::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point<T> {
     pub x: T,
     pub y: T
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Point {
            x,
            y
        }
    }
}

impl<T> Mul<T> for Point<T>
    where T: Mul<T, Output = T> + Copy
{
    type Output = Point<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Point {
            x: self.x * rhs,
            y: self.y * rhs
        }
    }
}

impl<T> From<(T, T)> for Point<T> {
    fn from(tuple: (T, T)) -> Self {
        Point {
            x: tuple.0,
            y: tuple.1
        }
    }
}

impl<T> std::fmt::Display for Point<T>
    where T: std::fmt::Display
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(x={}, y={})", self.x, self.y)
    }
}