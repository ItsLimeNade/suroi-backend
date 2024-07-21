use crate::typings::Orientation;
use super::vectors::Vec2D;
use super::math::{collisions, intersections, CollisionRecord, CollisionResponse, IntersectionResponse};

#[derive(Debug, Clone)]
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
    fn panic_unknown_subclass(other: &Hitbox);
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

    fn panic_unknown_subclass(other: &Hitbox) {
        panic!("Hitbox type CircleHitbox doesn't support this operation with hitbox type {:#?}", other);
    }

    fn collides_with(&self, other: &Hitbox) -> bool {
        match other {
            Hitbox::Circle(other) => {
                collisions::check_circles(other.position, other.radius, self.position, self.radius)
            },
            Hitbox::Rect(other) => {
                collisions::check_rects(other.min, other.max, self.position, self.radius)
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
                let col = intersections::circles(self.position, self.radius, other.position, other.radius);
                match col {
                    Some(collision) => self.position = self.position - Vec2D::scale(collision.dir, collision.pen),
                    _ => ()
                }
            },
            Hitbox::Rect(other) => {
                let col = intersections::rect_circle(other.min, other.max, self.position, self.radius);
                match col {
                    Some(collision) => self.position = self.position - Vec2D::scale(collision.dir, collision.pen),
                    _ => ()
                }
            },
            Hitbox::Group(other) => {
                for hitbox in &other.hitboxes {
                    if self.collides_with(hitbox) {
                        self.resolve_collision(hitbox)
                    }
                }
            },
            _ => CircleHitbox::panic_unknown_subclass(other)
        }
    }

    fn distance_to(&self, other: &Self) -> f64 {
        todo!()
    }

    fn transform(&self, pos: Vec2D, scale: f64, orientation: Orientation) -> Hitbox {
        todo!()
    }

    fn scale(&self, scale: f64) {
        todo!()
    }

    fn intersects_line(&self, a: Vec2D, b: Vec2D) -> IntersectionResponse {
        todo!()
    }

    fn random_point(&self) -> Vec2D {
        todo!()
    }

    fn as_rectangle(&self) -> RectangleHitbox {
        todo!()
    }

    fn is_vec_inside(&self, vec: Vec2D) -> bool {
        todo!()
    }

    fn get_center(&self) -> Vec2D {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct RectangleHitbox {
    min: Vec2D,
    max: Vec2D,
}

impl Collidable for RectangleHitbox {
    fn as_hitbox(&self) -> Hitbox {
        todo!()
    }

    fn collides_with(&self, other: &Hitbox) -> bool {
        todo!()
    }

    fn resolve_collision(&mut self, other: &Hitbox) {
        todo!()
    }

    fn distance_to(&self, other: &Self) -> f64 {
        todo!()
    }

    fn transform(&self, pos: Vec2D, scale: f64, orientation: Orientation) -> Hitbox {
        todo!()
    }

    fn scale(&self, scale: f64) {
        todo!()
    }

    fn intersects_line(&self, a: Vec2D, b: Vec2D) -> IntersectionResponse {
        todo!()
    }

    fn random_point(&self) -> Vec2D {
        todo!()
    }

    fn as_rectangle(&self) -> RectangleHitbox {
        todo!()
    }

    fn is_vec_inside(&self, vec: Vec2D) -> bool {
        todo!()
    }

    fn get_center(&self) -> Vec2D {
        todo!()
    }

    fn panic_unknown_subclass(other: &Hitbox) {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct PolygonHitbox {

}

impl Collidable for PolygonHitbox {
    fn as_hitbox(&self) -> Hitbox {
        todo!()
    }

    fn collides_with(&self, other: &Hitbox) -> bool {
        todo!()
    }

    fn resolve_collision(&mut self, other: &Hitbox) {
        todo!()
    }

    fn distance_to(&self, other: &Self) -> f64 {
        todo!()
    }

    fn transform(&self, pos: Vec2D, scale: f64, orientation: Orientation) -> Hitbox {
        todo!()
    }

    fn scale(&self, scale: f64) {
        todo!()
    }

    fn intersects_line(&self, a: Vec2D, b: Vec2D) -> IntersectionResponse {
        todo!()
    }

    fn random_point(&self) -> Vec2D {
        todo!()
    }

    fn as_rectangle(&self) -> RectangleHitbox {
        todo!()
    }

    fn is_vec_inside(&self, vec: Vec2D) -> bool {
        todo!()
    }

    fn get_center(&self) -> Vec2D {
        todo!()
    }

    fn panic_unknown_subclass(other: &Hitbox) {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct GroupHitbox {
    hitboxes: Vec<Hitbox>,
    position: Vec2D
}

impl Collidable for GroupHitbox {
    fn as_hitbox(&self) -> Hitbox {
        todo!()
    }

    fn collides_with(&self, other: &Hitbox) -> bool {
        todo!()
    }

    fn resolve_collision(&mut self, other: &Hitbox) {
        todo!()
    }

    fn distance_to(&self, other: &Self) -> f64 {
        todo!()
    }

    fn transform(&self, pos: Vec2D, scale: f64, orientation: Orientation) -> Hitbox {
        todo!()
    }

    fn scale(&self, scale: f64) {
        todo!()
    }

    fn intersects_line(&self, a: Vec2D, b: Vec2D) -> IntersectionResponse {
        todo!()
    }

    fn random_point(&self) -> Vec2D {
        todo!()
    }

    fn as_rectangle(&self) -> RectangleHitbox {
        todo!()
    }

    fn is_vec_inside(&self, vec: Vec2D) -> bool {
        todo!()
    }

    fn get_center(&self) -> Vec2D {
        todo!()
    }

    fn panic_unknown_subclass(other: &Hitbox) {
        todo!()
    }
}
