use std::ops::{Mul, Add, Sub, Div};

use crate::Body;

#[derive(Debug, Clone, Copy)]
pub struct Vec2 {
    x: f32,
    y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y:f32) -> Self {
        Self {x, y}
    }
}

impl  Mul<f32> for Vec2 {
    type Output = Vec2;

    fn mul(self, scalar:f32) -> Self::Output {
        Vec2 {
            x: self.x * scalar,
            y: self.x * scalar,
        }
    }
}

impl  Mul for Vec2 {
    type Output = Vec2;

    fn mul(self, other: Vec2) -> Self::Output {
        Vec2 {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, other: Self) -> Self::Output {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, other: Self) -> Self::Output {
        Vec2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }        
    }
}

impl PartialEq for Vec2 {
    fn eq(&self, other: &Self) -> bool {
        if self.x == other.x && self.y == self.y {
            return true
        }
        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiply_vec2d() {
        let v1 = Vec2::new(2.0, 5.0);
        let v2 = Vec2::new(5.0, 3.0);
        let res = v1 * v2;
        assert_eq!(res, Vec2::new(10.0, 15.0))
    }

    #[test]
    fn multiply_vec2d_scalar() {
        let v1 = Vec2::new(2.0, 5.0);
        let res = v1 * 6.0;
        assert_eq!(res, Vec2::new(12.0, 30.0))
    }

    #[test]
    fn add_vec2d() {
        let v1 = Vec2::new(2.0, 5.0);
        let v2 = Vec2::new(5.0, 3.0);
        let res = v1 + v2;
        assert_eq!(res, Vec2::new(7.0, 8.0))
    }

    #[test]
    fn sub_vec2d() {
        let v1 = Vec2::new(2.0, 5.0);
        let v2 = Vec2::new(5.0, 3.0);
        let res = v1 - v2;
        assert_eq!(res, Vec2::new(-3.0, 2.0))
    }
}