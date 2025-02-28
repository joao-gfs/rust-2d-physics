use std::ops::{Mul, MulAssign, Add, AddAssign, Sub, SubAssign, Div, DivAssign};

#[derive(Debug, Clone, Copy)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y:f32) -> Self {
        Self {x, y}
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

impl  Mul<f32> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs:f32) -> Self::Output {
        Vec2 {
            x: self.x * rhs,
            y: self.x * rhs,
        }
    }
}

impl MulAssign for Vec2 {
    fn mul_assign(&mut self, other: Self) {
        self.x = self.x * other.x;
        self.y = self.y * other.y;
    }
}

impl MulAssign<f32> for Vec2 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x = self.x * rhs;
        self.y = self.y * rhs;
    }
}

impl Div for Vec2 {
    type Output = Vec2;

    fn div(self, other: Vec2) -> Self::Output {
        Vec2 {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}

impl  Div<f32> for Vec2 {
    type Output = Vec2;

    fn div(self, rhs:f32) -> Self::Output {
        Vec2 {
            x: self.x / rhs,
            y: self.x / rhs,
        }
    }
}

impl DivAssign for Vec2 {
    fn div_assign(&mut self, other: Self) {
        self.x = self.x / other.x;
        self.y = self.y / other.y;
    }
}

impl DivAssign<f32> for Vec2 {
    fn div_assign(&mut self, rhs: f32) {
        self.x = self.x / rhs;
        self.y = self.y / rhs;
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

impl Add<f32> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: f32) -> Self::Output {
        Vec2{
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, other: Self) {
        self.x = self.x + other.x;
        self.y = self.y + other.y;
    }
}

impl AddAssign<f32> for Vec2 {
    fn add_assign(&mut self, rhs: f32) {
        self.x = self.x + rhs;
        self.y = self.y + rhs;
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

impl Sub<f32> for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: f32) -> Self::Output {
        Vec2{
            x: self.x - rhs,
            y: self.y - rhs,
        }
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, other: Self) {
        self.x = self.x - other.x;
        self.y = self.y - other.y;
    }
}

impl SubAssign<f32> for Vec2 {
    fn sub_assign(&mut self, rhs: f32) {
        self.x = self.x - rhs;
        self.y = self.y - rhs;
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
    fn mul_vec2d() {
        let v1 = Vec2::new(2.0, 5.0);
        let v2 = Vec2::new(5.0, 3.0);
        let res = v1 * v2;
        assert_eq!(res, Vec2::new(10.0, 15.0))
    }

    #[test]
    fn mul_vec2d_scalar() {
        let v1 = Vec2::new(2.0, 5.0);
        let res = v1 * 6.0;
        assert_eq!(res, Vec2::new(12.0, 30.0))
    }

    #[test]
    fn mul_assign_vec2d() {
        let mut v1 = Vec2::new(2.0, 6.0);
        let v2 = Vec2::new(5.0, 3.0);
        v1 *= v2;
        assert_eq!(v1, Vec2::new(10.0, 15.0));
    }

    #[test]
    fn mul_assign_vec2d_scalar() {
        let mut v1 = Vec2::new(2.0, 5.0);
        v1 *= 6.0;
        assert_eq!(v1, Vec2::new(12.0, 30.0))
    }

    #[test]
    fn div_vec2d() {
        let v1 = Vec2::new(2.0, 6.0);
        let v2 = Vec2::new(5.0, 3.0);
        let res = v1 / v2;
        assert_eq!(res, Vec2::new(0.4, 2.0))
    }

    #[test]
    fn div_vec2d_scalar() {
        let v1 = Vec2::new(2.0, 5.0);
        let res = v1 / 2.0;
        assert_eq!(res, Vec2::new(1.0, 2.5))
    }

    #[test]
    fn div_assign_vec2d() {
        let mut v1 = Vec2::new(2.0, 6.0);
        let v2 = Vec2::new(5.0, 3.0);
        v1 /= v2;
        assert_eq!(v1, Vec2::new(0.4, 2.0))
    }

    #[test]
    fn div_assign_vec2d_scalar() {
        let mut v1 = Vec2::new(2.0, 5.0);
        v1 /= 2.0;
        assert_eq!(v1, Vec2::new(1.0, 2.5))
    }

    #[test]
    fn add_vec2d() {
        let v1 = Vec2::new(2.0, 5.0);
        let v2 = Vec2::new(5.0, 3.0);
        let res = v1 + v2;
        assert_eq!(res, Vec2::new(7.0, 8.0))
    }

    #[test]
    fn add_vec2d_scalar() {
        let v1 = Vec2::new(2.0, 5.0);
        let res = v1 + 5.0;
        assert_eq!(res, Vec2::new(7.0, 10.0))
    }

    #[test]
    fn add_assign_vec2d() {
        let mut v1 = Vec2::new(2.0, 5.0);
        let v2 = Vec2::new(5.0, 3.0);
        v1 += v2;
        assert_eq!(v1, Vec2::new(7.0, 8.0))
    }

    #[test]
    fn add_assign_vec2d_scalar() {
        let mut v1 = Vec2::new(2.0, 5.0);
        v1 += 5.0;
        assert_eq!(v1, Vec2::new(7.0, 10.0))
    }

    #[test]
    fn sub_vec2d() {
        let v1 = Vec2::new(2.0, 5.0);
        let v2 = Vec2::new(5.0, 3.0);
        let res = v1 - v2;
        assert_eq!(res, Vec2::new(-3.0, 2.0))
    }

    #[test]
    fn sub_vec2d_scalar() {
        let v1 = Vec2::new(2.0, 5.0);
        let res = v1 - 1.0;
        assert_eq!(res, Vec2::new(1.0, 4.0))
    }

    #[test]
    fn sub_assign_vec2d() {
        let mut v1 = Vec2::new(2.0, 5.0);
        let v2 = Vec2::new(5.0, 3.0);
        v1 -= v2;
        assert_eq!(v1, Vec2::new(-3.0, 2.0))
    }

    #[test]
    fn sub_assign_vec2d_scalar() {
        let mut v1 = Vec2::new(2.0, 5.0);
        v1 -= 5.0;
        assert_eq!(v1, Vec2::new(-3.0, 2.0))
    }
}