use std::ops::{Add, AddAssign, Mul, Neg, Sub, SubAssign};

use serde::{Deserialize, Serialize};

use crate::direction::Direction;

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

impl<T: Copy + Neg<Output=T>> Point<T> {
    pub fn invert_y(&self) -> Self {
        Point { x: self.x, y: -self.y }
    }
}

impl<T: Copy + Neg<Output=T>> Neg for Point<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Point::new(-self.x, -self.y)
    }
}

impl<T: Copy + Add<Output=T>> Add for Point<T> {
    type Output = Point<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T: Copy + AddAssign> AddAssign for Point<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: Copy + Sub<Output=T>> Sub for Point<T> {
    type Output = Point<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Point::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T: Copy + SubAssign> SubAssign for Point<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T: Copy + Mul<Output=T>> Mul<T> for Point<T> {
    type Output = Point<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Point::new(self.x * rhs, self.y * rhs)
    }
}

pub type IntPoint = Point<i32>;
pub type FloatPoint = Point<f32>;

const DIRECTIONS: [[Option<Direction>; 3]; 3] = [
    [Some(Direction::LeftDown), Some(Direction::Down), Some(Direction::DownRight)],
    [Some(Direction::Left), None, Some(Direction::Right)],
    [Some(Direction::UpLeft), Some(Direction::Up), Some(Direction::RightUp)],
];

fn signum_plus_one(value: i32) -> usize {
    (value.signum() + 1) as usize
}

impl IntPoint {
    pub fn direction(&self) -> Option<Direction> {
        return DIRECTIONS[signum_plus_one(self.y)][signum_plus_one(self.x)];
    }
}

impl FloatPoint {
    pub fn direction(&self) -> Option<Direction> {
        (*self * 1000.).truncate().direction()
    }

    pub fn truncate(&self) -> Point<i32> {
        Point { x: self.x as i32, y: self.y as i32 }
    }
}

