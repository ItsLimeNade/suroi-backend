use crate::utils;
use utils::vectors::Vec2D;


#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn vec_create() {
        Vec2D {
            x: 5.0,
            y: 5.0
        };
        Vec2D::new(5.0,5.0);
    }

    #[test]
    pub fn vec_override_add() {
        let vec1: Vec2D = Vec2D::new(5.0, 5.0);
        let vec2: Vec2D = Vec2D::new(10.0, 10.0);

        assert_eq!(Vec2D::new(15.0, 15.0), vec1+vec2);
    }
}
