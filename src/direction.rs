use std::mem::swap;
use crate::Point;

#[derive(Debug, Clone, Copy)]
pub enum CardinalDirection {
    Up,
    Left,
    Down,
    Right,
}

impl CardinalDirection {
    pub fn to_direction(&self) -> Direction {
        match self {
            CardinalDirection::Up => Direction::Up,
            CardinalDirection::Left => Direction::Left,
            CardinalDirection::Down => Direction::Down,
            CardinalDirection::Right => Direction::Right,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    UpLeft,
    Left,
    LeftDown,
    Down,
    DownRight,
    Right,
    RightUp,
}

const HALF_SQRT_OF_TWO: f32 = std::f32::consts::SQRT_2 / 2.;

impl Direction {
    pub fn is_close_to(&self, other: &Direction) -> bool {
        let mut self_value = *self as i32;
        let mut other_value = *other as i32;
        if self_value < other_value {
            swap(&mut self_value, &mut other_value);
        }
        self_value - other_value <= 1 || (self_value == Direction::RightUp as i32 && other_value == Direction::Up as i32)
    }

    pub fn to_cardinal(&self) -> CardinalDirection {
        match self {
            Direction::Up => CardinalDirection::Up,
            Direction::UpLeft => CardinalDirection::Up,
            Direction::Left => CardinalDirection::Left,
            Direction::LeftDown => CardinalDirection::Left,
            Direction::Down => CardinalDirection::Down,
            Direction::DownRight => CardinalDirection::Down,
            Direction::Right => CardinalDirection::Right,
            Direction::RightUp => CardinalDirection::Right,
        }
    }

    pub fn to_unit_point(&self) -> Point<f32> {
        match self {
            Direction::Up => Point::new(0., 1.),
            Direction::UpLeft => Point::new(-HALF_SQRT_OF_TWO, HALF_SQRT_OF_TWO),
            Direction::Left => Point::new(-1., 0.),
            Direction::LeftDown => Point::new(-HALF_SQRT_OF_TWO, -HALF_SQRT_OF_TWO),
            Direction::Down => Point::new(0., -1.),
            Direction::DownRight => Point::new(HALF_SQRT_OF_TWO, -HALF_SQRT_OF_TWO),
            Direction::Right => Point::new(1., 0.),
            Direction::RightUp => Point::new(HALF_SQRT_OF_TWO, HALF_SQRT_OF_TWO),
        }
    }

    pub fn to_unit_integer_point(&self) -> Point<i32> {
        match self {
            Direction::Up => Point::new(0, 1),
            Direction::UpLeft => Point::new(-1, 1),
            Direction::Left => Point::new(-1, 0),
            Direction::LeftDown => Point::new(-1, -1),
            Direction::Down => Point::new(0, -1),
            Direction::DownRight => Point::new(1, -1),
            Direction::Right => Point::new(1, 0),
            Direction::RightUp => Point::new(1, 1),
        }
    }
}