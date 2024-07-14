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

    pub fn mult(&self, amplifier: f32) -> Vec2D {
        Vec2D {
            x: &self.x * amplifier,
            y: &self.y * amplifier
        }
    }
}




