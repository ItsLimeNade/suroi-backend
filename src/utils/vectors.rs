use std::ops::{Add, Mul, Sub, Neg};
use std::cmp::PartialEq;

use crate::typings::Orientation;

#[derive(Clone, Debug, Copy)]
pub struct Vec2D {
    pub x: f64,
    pub y: f64
}

impl PartialEq for Vec2D {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Add for Vec2D {
    type Output = Vec2D;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Vec2D {
    type Output = Vec2D;

    fn sub(self, other: Vec2D) -> Vec2D {
        Vec2D {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul for Vec2D {
    type Output = f64;

    fn mul(self, other: Vec2D) -> f64 {
        self.x * other.x + self.y * other.y
    }
}

impl Mul<f64> for Vec2D {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Vec2D {
            x: self.x * rhs,
            y: self.y * rhs
        }
    }
}

impl Neg for Vec2D {
    type Output = Vec2D;

    fn neg(self) -> Vec2D {
        Vec2D {
            x: -self.x,
            y: -self.y
        }
    }
}

impl Vec2D {
    pub fn new(x: f64, y: f64) -> Self {
        Vec2D {
            x,
            y
        }
    }

    pub fn scale(self, scalar: f64) -> Self {
        Vec2D {
            x: self.x * scalar,
            y: self.y * scalar
        }
    }

    pub fn clone(self) -> Self {
        Vec2D {
            x: self.x,
            y: self.y
        }
    }

    pub fn rotate(self, angle: f64) -> Self {
        let cos: f64 = angle.cos();
        let sin: f64 = angle.sin();
        Vec2D {
            x: self.x * cos - self.y * sin,
            y: self.x * sin + self.y * cos
        }
    }

    pub fn squared_length(self) -> f64 {
        self.x * self.x + self.y * self.y
    }

    pub fn length(self) -> f64 {
        f64::sqrt(self.squared_length())
    }

    pub fn direction(self) -> f64 {
        f64::atan2(self.y, self.x)
    }

    pub fn angle(self, vec2: Vec2D) -> f64  {
        f64::acos((self.x * vec2.x + self.y * vec2.y) / f64::sqrt(self.length() * vec2.length()))
    }

    pub fn lerp(self, end: Vec2D, interp_factor: f64) -> Self {
        self * (1.0 - interp_factor) + end * interp_factor
    }

    pub fn project(self, vec2: Vec2D) -> Self {
        vec2 * (self * vec2 / vec2.squared_length())
    }

    pub fn normalize(self, fallback: Option<Vec2D>) -> Self {
        let fallback: Vec2D = fallback.unwrap_or(Vec2D::new(1.0, 0.0));
        let len = self.length();

        if len > 0.000001 {
            Vec2D {
                x: self.x / len,
                y: self.y / len
            }
        } else {
            fallback
        }
    }

    pub fn equals(self, vec2: Vec2D, epsilon: Option<f64>) -> bool {
        let epsilon: f64 = epsilon.unwrap_or(0.001);
        f64::abs(self.x - vec2.x) <= epsilon && f64::abs(self.y - vec2.y) <= epsilon
    }

    pub fn from_polar(angle: f64, magnitude: Option<f64>) -> Self {
        let magnitude: f64 = magnitude.unwrap_or(1.0);
        Vec2D {
            x: f64::cos(angle) * magnitude,
            y: f64::sin(angle) * magnitude
        }
    }

    pub fn add_adjust(self, pos2: Vec2D, orientation: Orientation) -> Vec2D {
        self + pos2.rotate(orientation.to_angle())
    }
}
