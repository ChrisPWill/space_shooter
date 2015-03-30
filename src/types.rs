use std::ops::{Add, Mul};

#[derive(Clone)]
pub struct Vec2F {
    pub x: f64,
    pub y: f64,
}

impl Add for Vec2F {
    type Output = Vec2F;

    fn add(self, _rhs: Vec2F) -> Vec2F {
        Vec2F{x: self.x + _rhs.x, y: self.y + _rhs.y}
    }
}

impl Mul for Vec2F {
    type Output = Vec2F;

    fn mul(self, _rhs: Vec2F) -> Vec2F {
        Vec2F{x: self.x*_rhs.x, y: self.y*_rhs.y}
    }
}

impl Mul<f64> for Vec2F {
    type Output = Vec2F;

    fn mul(self, _rhs: f64) -> Vec2F {
        Vec2F{x: self.x*_rhs, y: self.y*_rhs}
    }
}

