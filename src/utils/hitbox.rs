use crate::typings::Orientation;
use super::vectors::Vec2D;
use super::math::{collisions, intersections, CollisionRecord, CollisionResponse, IntersectionResponse};

#[derive(Debug)]
pub enum Hitbox {
    Circle(CircleHitbox),
    Rect(RectangleHitbox),
    Group(GroupHitbox),
    Polygon(PolygonHitbox)
}

pub trait Collidable {
    fn as_hitbox(&self) -> Hitbox;
    fn collides_with(&self, other: &Hitbox) -> bool;
    fn resolve_collision(&mut self, other: &Hitbox);
    fn distance_to(&self, other: &Self) -> f64;
    fn transform(&self, pos: Vec2D, scale: f64, orientation: Orientation) -> Hitbox;
    fn scale(&self, scale: f64);
    fn intersects_line(&self, a:Vec2D, b:Vec2D) -> IntersectionResponse;
    fn random_point(&self) -> Vec2D;
    fn as_rectangle(&self) -> RectangleHitbox;
    fn is_vec_inside(&self, vec: Vec2D) -> bool;
    fn get_center(&self) -> Vec2D;
}

#[derive(Debug, Clone)]
pub struct CircleHitbox {
    position: Vec2D,
    radius: f64,
}

impl Collidable for CircleHitbox {

    fn as_hitbox(&self) -> Hitbox {
        Hitbox::Circle(self.clone())
    }

    fn collides_with(&self, other: &Hitbox) -> bool {
        match other {
            Hitbox::Circle(other) => {
                collisions::circle_collision(other.position, other.radius, self.position, self.radius)
            },
            Hitbox::Rect(other) => {
                collisions::rectangle_collision(other.min, other.max, self.position, self.radius)
            },
            Hitbox::Group(other) => {
                other.collides_with(&self.as_hitbox())
            },
            Hitbox::Polygon(other) => {
                other.collides_with(&self.as_rectangle().as_hitbox())
            }
        }
    }
    fn resolve_collision(&mut self, other: &Hitbox) {
        match other {
            Hitbox::Circle(other) => {
                let col = intersections::circle_circle(self.position, self.radius, other.position, other.radius);
                match col {
                    Some(collision) => self.position = self.position - Vec2D::scale(collision.dir, collision.pen),
                    _ => ()
                }
            },
            Hitbox::Rect(other) => {
                let col = collisions::
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

impl Collidable for RectangleHitbox {

}

#[derive(Debug, Clone)]
pub struct PolygonHitbox {

}

impl Collidable for PolygonHitbox {

}

#[derive(Debug, Clone)]
pub struct GroupHitbox {

}

impl Collidable for GroupHitbox {

}
