#[derive(Debug)]
pub struct Vec2D {
    x: f32,
    y: f32
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
        f32::sqrt(vector.squared_length(vector))
    }
}





