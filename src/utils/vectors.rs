use std::ops::{Add, Mul, Sub, Neg, MulAssign};

#[derive(Clone, Debug)]
pub struct Vec2D {
    pub x: f64,
    pub y: f64
}

impl Add for Vec2D {
    type Output = Vec2D;

    fn add(self, other: Vec2D) -> Vec2D {
        Vec2D {
            x: self.x + other.x,
            y: self.y + other.y,
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

impl MulAssign<f64> for Vec2D {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
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

    pub fn add(vector1: &Vec2D, vector2: &Vec2D) -> Self {
        Vec2D {
            x: &vector1.x + &vector2.x,
            y: &vector1.y + &vector2.y
        }
    }

    pub fn sub(vector1: &Vec2D, vector2: &Vec2D) -> Self {
        Vec2D {
            x: &vector1.x - &vector2.x,
            y: &vector1.y - &vector2.y
        }
    }

    pub fn scale(&self, amplifier: f64) -> Self {
        Vec2D {
            x: &self.x * amplifier,
            y: &self.y * amplifier
        }
    }

    pub fn clone(&self) -> Self {
        Vec2D {
            x: self.x,
            y: self.y
        }
    }

    pub fn rotate(vector: &Vec2D, angle: f64) -> Self {
        let cos: f64 = f64::cos(angle);
        let sin: f64 = f64::sin(angle);
        Vec2D {
            x: vector.x * cos - vector.y * sin,
            y: vector.x * sin + vector.y * cos
        }
    }

    pub fn squared_length(vector: &Vec2D) -> f64 {
        vector.x * vector.x + vector.y * vector.y
    }

    pub fn length(vector: &Vec2D) -> f64 {
        f64::sqrt(Vec2D::squared_length(vector))
    }

    pub fn dot_product(vec1: &Vec2D, vec2: &Vec2D) -> f64 {
        vec1.x * vec2.x + vec1.y * vec2.y
    }

    pub fn direction(vector: &Vec2D) -> f64 {
        f64::atan2(vector.y, vector.x)
    }

    pub fn invert(vector: &Vec2D) -> Vec2D {
        Vec2D {
            x: -vector.x,
            y: -vector.y
        }
    }

    pub fn angle(vec1: &Vec2D, vec2: &Vec2D) -> f64  {
        f64::acos((vec1.x * vec2.x + vec1.y * vec2.y) / f64::sqrt(Vec2D::length(vec1) * Vec2D::length(vec2)))
    }

    pub fn lerp(start: &Vec2D, end: &Vec2D, interp_factor: &f64) -> Self {
        Vec2D::scale(start, 1.0 - interp_factor) + Vec2D::scale(end, *interp_factor)
    }

    pub fn project(vec1: &Vec2D, vec2: &Vec2D) -> Self {
        Vec2D::scale(vec2, Vec2D::dot_product(vec1, vec2) / Vec2D::squared_length(vec2))
    }

    pub fn normalize(vector: &Vec2D, fallback: &Option<Vec2D>) -> Self {
        let fallback: Vec2D = match fallback {
            Some(thing) => thing.clone(),
            None => Vec2D::new(1.0, 0.0),
        };
        let len = Vec2D::length(vector);

        if len > f64::from(0.000001) {
            Vec2D {
                x: vector.x / len,
                y: vector.y / len
            }
        } else {
            fallback
        }
    }

    pub fn equals(vec1: &Vec2D, vec2: &Vec2D, epsilon: &Option<f64>) -> bool {
        let epsilon: f64 = match epsilon {
            Some(eps) => eps.clone(),
            None => f64::from(0.001),
        };
        f64::abs(vec1.x - vec2.x) <= epsilon && f64::abs(vec1.y - vec2.y) <=epsilon
    }

    pub fn from_polar(angle: &f64, magnitude: &Option<f64>) -> Self {
        let magnitude: f64 = match magnitude {
            Some(mag) => mag.clone(),
            None => f64::from(1.0),
        };
        Vec2D {
            x: f64::cos(*angle) * magnitude,
            y: f64::sin(*angle) * magnitude
        }
    }
}
