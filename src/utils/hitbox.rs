use crate::typings::Orientation;
use super::vectors::Vec2D;
use super::math::{collision, CollisionRecord, CollisionResponse, IntersectionResponse};

#[derive(Debug)]
pub enum Hitbox {
    Circle(CircleHitbox),
    Rect(RectangleHitbox),
    Group(GroupHitbox),
    Polygon(PolygonHitbox)
}

pub trait Collidable {
    fn collides_with(&self, other: &Hitbox) -> bool;
    fn resolve_collision(&mut self, other: &Hitbox);
    fn distance_to(&self, other: &Self) -> f64;
    fn transform(&self, pos: Vec2D, scale: f64, orientation: Orientation) -> Hitbox;
    fn scale(&self, scale: f64);
    fn intersects_line(&self, a:Vec2D, b:Vec2D) -> IntersectionResponse;
    fn random_point(&self) -> Vec2D;
    fn to_rectangle(&self) -> RectangleHitbox;
    fn is_vec_inside(&self, vec: Vec2D) -> bool;
    fn get_center(&self) -> Vec2D;
}

#[derive(Debug, Clone)]
pub struct CircleHitbox {
    position: Vec2D,
    radius: f64,
}

impl Collidable for CircleHitbox {
    fn collides_with(&self, other: &Hitbox) -> bool {
        match other {
            Hitbox::Circle(other) => {
                collision::circle_collision(other.position, other.radius, self.position, self.radius)
            },
            Hitbox::Rect(other) => {

            },
            Hitbox::Group(other) => {

            },
            Hitbox::Polygon(other) => {

            }
        }
    }
    fn resolve_collision(&mut self, other: &Hitbox) {
        match other {
            Hitbox::Circle(other) => {
                let col = collision::circle_circle_intersection(self.position, self.radius, other.position, other.radius);
                match col {
                    Some(collision) => self.position = self.position - Vec2D::scale(collision.dir, collision.pen),
                    _ => ()
                }
            },
            Hitbox::Rect(other) => {

            },
            Hitbox::Group(other) => {

            },
            Hitbox::Polygon(other) => {

            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct RectangleHitbox {
    min: Vec2D,
    max: Vec2D,
}

#[derive(Debug, Clone)]
pub struct PolygonHitbox {

}

#[derive(Debug, Clone)]
pub struct GroupHitbox {

}
