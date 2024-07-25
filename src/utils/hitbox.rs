use super::math::{
    collisions, collisions::distances, geometry, intersections, CollisionRecord,
    IntersectionResponse,
};
use super::random::random_point_in_circle;
use super::vectors::Vec2D;
use crate::typings::Orientation;

#[derive(Debug, Clone)]
pub enum Hitbox {
    Circle(CircleHitbox),
    Rect(RectangleHitbox),
    Group(GroupHitbox),
    Polygon(PolygonHitbox),
}

pub trait Collidable {
    fn as_hitbox(&self) -> Hitbox;
    fn collides_with(&self, other: &Hitbox) -> bool;
    fn resolve_collision(&mut self, other: &Hitbox);
    fn distance_to(&self, other: &Hitbox) -> Option<CollisionRecord>;
    fn transform(&self, pos: Vec2D, scale: Option<f64>, orientation: Option<Orientation>) -> Self;
    fn scale(&mut self, scale: f64);
    fn intersects_line(&self, a: Vec2D, b: Vec2D) -> Option<IntersectionResponse>;
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
        panic!(
            "Hitbox type CircleHitbox doesn't support this operation with hitbox type {:#?}",
            other
        );
    }

    fn collides_with(&self, other: &Hitbox) -> bool {
        match other {
            Hitbox::Circle(other) => {
                collisions::check_circles(other.position, other.radius, self.position, self.radius)
            }
            Hitbox::Rect(other) => {
                collisions::check_rect_circle(other.min, other.max, self.position, self.radius)
            }
            Hitbox::Group(other) => other.collides_with(&self.as_hitbox()),
            Hitbox::Polygon(other) => other.collides_with(&self.as_rectangle().as_hitbox()),
        }
    }

    fn resolve_collision(&mut self, other: &Hitbox) {
        match other {
            Hitbox::Circle(other) => {
                if let Some(collision) =
                    intersections::circles(self.position, self.radius, other.position, other.radius)
                {
                    self.position = self.position - (collision.dir * collision.pen)
                }
            }
            Hitbox::Rect(other) => {
                if let Some(collision) =
                    intersections::rect_circle(other.min, other.max, self.position, self.radius)
                {
                    self.position = self.position - (collision.dir * collision.pen)
                }
            }
            Hitbox::Group(other) => {
                for hitbox in &other.hitboxes {
                    if self.collides_with(hitbox) {
                        self.resolve_collision(hitbox)
                    }
                }
            }
            _ => CircleHitbox::panic_unknown_subclass(other),
        }
    }

    fn distance_to(&self, other: &Hitbox) -> Option<CollisionRecord> {
        match other {
            Hitbox::Circle(other) => Some(distances::circles(
                other.position,
                other.radius,
                self.position,
                self.radius,
            )),
            Hitbox::Rect(other) => Some(distances::circle_rect(
                other.min,
                other.max,
                self.position,
                self.radius,
            )),
            _ => {
                CircleHitbox::panic_unknown_subclass(other);
                None
            }
        }
    }

    fn transform(&self, pos: Vec2D, scale: Option<f64>, orientation: Option<Orientation>) -> Self {
        CircleHitbox {
            position: Vec2D::add_adjust(pos, self.position, orientation.unwrap_or(Orientation::Up)),
            radius: self.radius * scale.unwrap_or(1.0),
        }
    }

    fn scale(&mut self, scale: f64) {
        self.radius *= scale;
    }

    fn intersects_line(&self, a: Vec2D, b: Vec2D) -> Option<IntersectionResponse> {
        intersections::line_circle(a, b, self.position, self.radius)
    }

    fn random_point(&self) -> Vec2D {
        random_point_in_circle(self.position, None, self.radius)
    }

    fn as_rectangle(&self) -> RectangleHitbox {
        RectangleHitbox {
            min: Vec2D {
                x: self.position.x - self.radius,
                y: self.position.y - self.radius,
            },
            max: Vec2D {
                x: self.position.x + self.radius,
                y: self.position.y + self.radius,
            },
        }
    }

    fn is_vec_inside(&self, vec: Vec2D) -> bool {
        geometry::distance(vec, self.position) < self.radius
    }

    fn get_center(&self) -> Vec2D {
        self.position
    }
}

#[derive(Debug, Clone)]
pub struct RectangleHitbox {
    min: Vec2D,
    max: Vec2D,
}

impl RectangleHitbox {
    pub fn from_line(a: Vec2D, b: Vec2D) -> RectangleHitbox {
        RectangleHitbox {
            min: Vec2D {
                x: a.x.min(b.x),
                y: a.y.min(b.y),
            },
            max: Vec2D {
                x: a.x.max(b.x),
                y: a.y.max(b.y),
            },
        }
    }

    pub fn from_rect(width: f64, height: f64, center: Option<Vec2D>) -> RectangleHitbox {
        let size = Vec2D::new(width / 2.0, height / 2.0);
        let center = center.unwrap_or(Vec2D::new(0.0, 0.0));

        RectangleHitbox {
            min: center - size,
            max: center + size,
        }
    }
}

impl Collidable for RectangleHitbox {
    fn as_hitbox(&self) -> Hitbox {
        Hitbox::Rect(self.clone())
    }

    fn collides_with(&self, other: &Hitbox) -> bool {
        match other {
            Hitbox::Circle(other) => {
                collisions::check_rect_circle(self.min, self.max, other.position, other.radius)
            }
            Hitbox::Rect(other) => {
                collisions::check_rects(other.min, other.max, self.min, self.max)
            }
            Hitbox::Polygon(other) => other.collides_with(&self.as_hitbox()),
            Hitbox::Group(other) => other.collides_with(&self.as_hitbox()),
        }
    }

    fn resolve_collision(&mut self, other: &Hitbox) {
        match other {
            Hitbox::Circle(other) => {
                if let Some(collision) =
                    intersections::rect_circle(self.min, self.max, other.position, other.radius)
                {
                    let rect = self.transform(collision.dir * -collision.pen, None, None);
                    self.max = rect.max;
                    self.min = rect.min;
                }
            }
            Hitbox::Rect(other) => {
                if let Some(collision) = intersections::rects(self.min, self.max, other.min, other.max)
                {
                    let rect = self.transform(collision.dir * -collision.pen, None, None);
                    self.min = rect.min;
                    self.max = rect.max
                }
            }
            Hitbox::Group(other) => {
                for hitbox in &other.hitboxes {
                    if self.collides_with(hitbox) {
                        self.resolve_collision(hitbox)
                    }
                }
            }
            _ => RectangleHitbox::panic_unknown_subclass(other),
        }
    }

    fn distance_to(&self, other: &Hitbox) -> Option<CollisionRecord> {
        todo!()
    }

    fn transform(&self, pos: Vec2D, scale: Option<f64>, orientation: Option<Orientation>) -> Self {
        todo!()
    }

    fn scale(&mut self, scale: f64) {
        todo!()
    }

    fn intersects_line(&self, a: Vec2D, b: Vec2D) -> Option<IntersectionResponse> {
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
pub struct PolygonHitbox {}

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

    fn distance_to(&self, other: &Hitbox) -> Option<CollisionRecord> {
        todo!()
    }

    fn transform(&self, pos: Vec2D, scale: Option<f64>, orientation: Option<Orientation>) -> Self {
        todo!()
    }

    fn scale(&mut self, scale: f64) {
        todo!()
    }

    fn intersects_line(&self, a: Vec2D, b: Vec2D) -> Option<IntersectionResponse> {
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
    position: Vec2D,
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

    fn distance_to(&self, other: &Hitbox) -> Option<CollisionRecord> {
        todo!()
    }

    fn transform(&self, pos: Vec2D, scale: Option<f64>, orientation: Option<Orientation>) -> Self {
        todo!()
    }

    fn scale(&mut self, scale: f64) {
        todo!()
    }

    fn intersects_line(&self, a: Vec2D, b: Vec2D) -> Option<IntersectionResponse> {
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
