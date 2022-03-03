use serde::{Serialize, Deserialize};
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Point<T: Copy> {
    pub x: T,
    pub y: T,
}

impl<T: Copy> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
}

impl<T: Copy + Add<Output=T>> Add for Point<T> {
    type Output = Point<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T: Copy + Sub<Output=T>> Sub for Point<T> {
    type Output = Point<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Point::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T: Copy + Mul<Output=T>> Mul<T> for Point<T> {
    type Output = Point<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Point::new(self.x * rhs, self.y * rhs)
    }
}