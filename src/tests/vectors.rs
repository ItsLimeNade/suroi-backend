use crate::utils::vectors::Vec2D;

#[cfg(test)]
pub mod vectors {
    use super::*;

    #[test]
    pub fn create() {
        Vec2D {
            x: 5.0,
            y: 5.0
        };
        Vec2D::new(5.0,5.0);
    }

    #[test]
    pub fn add() {
        let vec1: Vec2D = Vec2D::new(5.0, 5.0);
        let vec2: Vec2D = Vec2D::new(10.0, 10.0);

        assert_eq!(Vec2D::new(15.0, 15.0), vec1+vec2);
    }

    #[test]
    pub fn subsctract() {
        let vec1: Vec2D = Vec2D::new(5.0, 5.0);
        let vec2: Vec2D = Vec2D::new(10.0, 10.0);

        assert_eq!(Vec2D::new(5.0,5.0), vec2 - vec1);
    }

    #[test]
    pub fn dot_pr() {
        let vec1: Vec2D = Vec2D::new(5.0, 5.0);
        let vec2: Vec2D = Vec2D::new(10.0, 10.0);

        assert_eq!(5.0*10.0*2.0, vec1 * vec2);
    }

    #[test]
    pub fn eq() {
        let vec1: Vec2D = Vec2D::new(5.0, 5.0);
        let vec2: Vec2D = Vec2D::new(5.0, 5.0);
        let vec3: Vec2D = Vec2D::new(10.0, 10.0);

        assert!(vec1 == vec2);
        assert!(vec1 != vec3);

        assert!(Vec2D::equals(vec1, vec2, None));
        assert!(!Vec2D::equals(vec1, vec3, None));

    }

    #[test]
    pub fn scale() {
        let mut vec1: Vec2D = Vec2D::new(5.0, 5.0);
        let vec2: Vec2D = Vec2D::new(5.0, 5.0);

        assert_eq!(Vec2D::new(10.0, 10.0), vec1 * 2.0);
        assert_eq!(Vec2D::new(10.0, 10.0), Vec2D::scale(vec2, 2.0));
    }

    #[test]
    pub fn invert() {
        let vec1: Vec2D = Vec2D::new(5.0, 5.0);
        assert_eq!(Vec2D::new(-5.0,-5.0), -vec1);
    }

}
