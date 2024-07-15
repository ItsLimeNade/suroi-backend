use std::ops::{Add, Mul, Sub, Neg};

#[derive(Debug)]
pub struct Vec2D {
    x: f32,
    y: f32
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
    type Output = f32;

    fn mul(self, other: Vec2D) -> f32 {
        self.x * other.x + self.y * other.y
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
    pub fn new(x: f32, y: f32) -> Self {
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

    pub fn scale(&self, amplifier: f32) -> Self {
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

    pub fn rotate(vector: &Vec2D, angle: f32) -> Self {
        let cos: f32 = f32::cos(angle);
        let sin: f32 = f32::sin(angle);
        Vec2D {
            x: vector.x * cos - vector.y * sin,
            y: vector.x * sin + vector.y * cos
        }
    }

    pub fn squared_length(vector: &Vec2D) -> f32 {
        vector.x * vector.x + vector.y * vector.y
    }

    pub fn length(vector: &Vec2D) -> f32 {
        f32::sqrt(Vec2D::squared_length(vector))
    }

    pub fn dot_product(vec1: &Vec2D, vec2: &Vec2D) -> f32 {
        vec1.x * vec2.x + vec1.y * vec2.y
    }

    pub fn direction(vector: &Vec2D) -> f32 {
        f32::atan2(vector.y, vector.x)
    }

    pub fn invert(vector: &Vec2D) -> Vec2D {
        Vec2D {
            x: -vector.x,
            y: -vector.y
        }
    }

    pub fn ang_vec(vec1: &Vec2D, vec2: &Vec2D)  {
        print!("lol")
    }
}
