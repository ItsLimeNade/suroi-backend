use core::f64;

use super::math::{
    collisions, collisions::distances, geometry, intersections, CollisionRecord,
    IntersectionResponse,
};
use super::random::{random_point_in_circle, random_float, random_item};
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
    fn resolve_collision(&mut self, other: &mut Hitbox);
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

    fn resolve_collision(&mut self, other: &mut Hitbox) {
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
                for hitbox in &mut other.hitboxes {
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

    fn resolve_collision(&mut self, other: &mut Hitbox) {
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
                for hitbox in &mut other.hitboxes {
                    if self.collides_with(hitbox) {
                        self.resolve_collision(hitbox)
                    }
                }
            }
            _ => RectangleHitbox::panic_unknown_subclass(other),
        }
    }

    fn distance_to(&self, other: &Hitbox) -> Option<CollisionRecord> {
        match other {
            Hitbox::Circle(other) => {
                Some(distances::circle_rect(self.min, self.max, other.position, other.radius))
            },
            Hitbox::Rect(other) => {
                Some(distances::rects(other.min, other.max, self.min, self.max))
            }
            _ => {
                RectangleHitbox::panic_unknown_subclass(other);
                None
            }
        }
    }

    fn transform(&self, pos: Vec2D, scale: Option<f64>, orientation: Option<Orientation>) -> Self {
        let mut smol_rect = geometry::Rectangle {
            min: self.min,
            max: self.max
        };
        let rect = geometry::Rectangle::transform(&mut smol_rect, pos, scale.unwrap_or(1.0), orientation.unwrap_or(Orientation::Up));
        RectangleHitbox {
            min: rect.min,
            max: rect.max
        }
    }

    fn scale(&mut self, scale: f64) {
        let center_x = (self.min.x + self.max.x) / 2.0_f64;
        let center_y = (self.min.y + self.max.y) / 2.0_f64;

        self.min = Vec2D {
            x: (self.min.x - center_x) * scale + center_x,
            y: (self.min.y - center_y) * scale + center_y
        };

        self.max = Vec2D {
            x: (self.max.x - center_x) * scale + center_x,
            y: (self.max.y - center_y) * scale + center_y
        };
    }

    fn intersects_line(&self, a: Vec2D, b: Vec2D) -> Option<IntersectionResponse> {
        intersections::line_rect(a, b, self.min, self.max)
    }

    fn random_point(&self) -> Vec2D {
        Vec2D {
            x: random_float(self.min.x, self.max.x),
            y: random_float(self.min.y, self.max.y)
        }
    }

    fn as_rectangle(&self) -> RectangleHitbox {
        self.clone()
    }

    fn is_vec_inside(&self, vec: Vec2D) -> bool {
        vec.x > self.min.x && vec.y > self.min.y && vec.x < self.max.x && vec.y < self.max.y
    }

    fn get_center(&self) -> Vec2D {
        Vec2D {
            x: self.min.x + ((self.max.x - self.min.x) / 2.0_f64),
            y: self.min.y + ((self.max.y - self.min.y) / 2.0_f64 )
        }
    }

    fn panic_unknown_subclass(other: &Hitbox) {
        panic!(
            "Hitbox type RectangleHitbox doesn't support this operation with hitbox type {:#?}",
            other
        );
    }
}

#[derive(Debug, Clone)]
pub struct PolygonHitbox {
    points: Vec<Vec2D>,
    center: Vec2D
}

impl Collidable for PolygonHitbox {
    fn as_hitbox(&self) -> Hitbox {
        Hitbox::Polygon(self.clone())
    }

    fn collides_with(&self, other: &Hitbox) -> bool {
        match other {
            Hitbox::Rect(other) => {
                if self.is_vec_inside(other.min) || self.is_vec_inside(other.max) {
                    return true;
                }
                for (i, a) in self.points.iter().enumerate() {
                    if other.is_vec_inside(*a) { return true; }
                    let j = (i + 1) % self.points.len();
                    let b = self.points[j as usize];
                    if intersections::line_rect_test(b, *a, other.min, other.max) {
                        return true;
                    }
                }
                return false;
            }
            _ => {
                Self::panic_unknown_subclass(other);
                return false;
            }
        }
    }

    fn resolve_collision(&mut self, other: &mut Hitbox) {
        Self::panic_unknown_subclass(other);
    }

    fn distance_to(&self, other: &Hitbox) -> Option<CollisionRecord> {
        Self::panic_unknown_subclass(other);
        None
    }

    fn transform(&self, pos: Vec2D, scale: Option<f64>, orientation: Option<Orientation>) -> Self {
        PolygonHitbox {
            points: self.points.iter().map(|point| pos.add_adjust(*point, orientation.unwrap_or(Orientation::Up)) * scale.unwrap_or(1.0)).collect(),
            center: self.center
        }
    }

    fn scale(&mut self, scale: f64) {
        for point in self.points.iter_mut() {
            *point = *point * scale;
        }
    }

    fn intersects_line(&self, a: Vec2D, b: Vec2D) -> Option<IntersectionResponse> {
        panic!("Operation not supported");
    }

    fn random_point(&self) -> Vec2D {
        let rect = self.as_rectangle();
        let mut point: Vec2D;
        loop {
            point = rect.random_point();
            if self.is_vec_inside(point) { break; }
        }
        point
    }

    fn as_rectangle(&self) -> RectangleHitbox {
        let mut min = Vec2D::new(f64::INFINITY, f64::INFINITY);
        let mut max = Vec2D::new(0.0, 0.0);
        for point in self.points.iter() {
            min.x = min.x.min(point.x);
            min.y = min.y.min(point.y);
            max.x = max.x.max(point.x);
            max.y = max.y.max(point.y);
        }
        RectangleHitbox {
            min,
            max
        }
    }

    fn is_vec_inside(&self, vec: Vec2D) -> bool {
        let mut inside = false;
        let count = self.points.len();
        let mut j = count - 1;
        for i in 0..count {
            let pi = self.points[i as usize];
            let pj = self.points[j as usize];
            if (pi.y > vec.y) != (pj.y > vec.y) && vec.x < (pj.x - pi.x) * (vec.y - pi.y) / (pj.y - pi.y) + pi.x {
                inside = !inside;
            }
            j = i;
        }
        inside
    }

    fn get_center(&self) -> Vec2D {
        self.as_rectangle().get_center()
    }

    fn panic_unknown_subclass(other: &Hitbox) {
        panic!(
            "Hitbox type PolygonHitbox doesn't support this operation with hitbox type {:#?}",
            other
        );
    }
}

#[derive(Debug, Clone)]
pub struct GroupHitbox {
    hitboxes: Vec<Hitbox>,
    position: Vec2D,
}

impl GroupHitbox {
    pub fn new(hitboxes: Vec<Hitbox>) -> GroupHitbox {
        GroupHitbox {
            hitboxes,
            position: Vec2D::new(0.0, 0.0)
        }
    }
}

impl Collidable for GroupHitbox {
    fn as_hitbox(&self) -> Hitbox {
        Hitbox::Group(self.clone())
    }
    fn collides_with(&self, other: &Hitbox) -> bool {
        self.hitboxes.iter().any(|hitbox| match hitbox {
            Hitbox::Circle(hitbox) => hitbox.collides_with(other),
            Hitbox::Rect(hitbox) => hitbox.collides_with(other),
            Hitbox::Polygon(hitbox) => hitbox.collides_with(other),
            Hitbox::Group(hitbox) => hitbox.collides_with(other),
        })
    }

    fn resolve_collision(&mut self, other: &mut Hitbox) {
        match other {
            Hitbox::Circle(other) => other.resolve_collision(&mut self.as_hitbox()),
            Hitbox::Rect(other) => other.resolve_collision(&mut self.as_hitbox()),
            Hitbox::Polygon(other) => other.resolve_collision(&mut self.as_hitbox()),
            Hitbox::Group(other) => other.resolve_collision(&mut self.as_hitbox()),
        }
    }

    fn distance_to(&self, other: &Hitbox) -> Option<CollisionRecord> {
        let mut distance = f64::MAX;
        let mut record = CollisionRecord {
            collided: false,
            distance: f64::MAX
        };

        for hitbox in self.hitboxes.iter() {
            let new_record: CollisionRecord;

            match hitbox {
                Hitbox::Circle(hitbox) => {
                    match other {
                        Hitbox::Circle(other) => {
                            new_record = distances::circles(other.position, other.radius, hitbox.position, hitbox.radius);
                        },
                        Hitbox::Rect(other) => {
                            new_record = distances::circle_rect(other.min, other.max, hitbox.position, hitbox.radius);
                        },
                        _ => {
                            Self::panic_unknown_subclass(other);
                            return None;
                        }
                    }
                },
                Hitbox::Rect(hitbox) => {
                    match  other {
                        Hitbox::Circle(other) => {
                            new_record = distances::circle_rect(hitbox.min, hitbox.max, other.position, other.radius);
                        },
                        Hitbox::Rect(other) => {
                            new_record = distances::rects(other.min, other.max, hitbox.min, hitbox.max)
                        },
                        _ => {
                            Self::panic_unknown_subclass(other);
                            return None;
                        }
                    }
                },
                _ => {
                    Self::panic_unknown_subclass(hitbox);
                    return None;
                }
            }

            if new_record.distance < distance {
                record = new_record;
                distance = new_record.distance;
            }
        }

        //TODO: I don't know if this is the right way to deal with this.
        Some(record)
    }

    fn transform(&self, pos: Vec2D, scale: Option<f64>, orientation: Option<Orientation>) -> Self {
        GroupHitbox {
            hitboxes: self.hitboxes.iter().map(|hitbox| {
                match hitbox {
                    Hitbox::Circle(circle) => Hitbox::Circle(circle.transform(pos, scale, orientation)),
                    Hitbox::Rect(rect) => Hitbox::Rect(rect.transform(pos, scale, orientation)),
                    Hitbox::Polygon(polygon) => Hitbox::Polygon(polygon.transform(pos, scale, orientation)),
                    Hitbox::Group(group) => Hitbox::Group(group.transform(pos, scale, orientation)),
                }
            }).collect(),
            position: pos,
        }
    }


    fn scale(&mut self, scale: f64) {
        for hitbox in self.hitboxes.iter_mut() {
            match hitbox {
                Hitbox::Circle(hitbox) => hitbox.scale(scale),
                Hitbox::Rect(hitbox) => hitbox.scale(scale),
                Hitbox::Polygon(hitbox) => hitbox.scale(scale),
                Hitbox::Group(hitbox) => hitbox.scale(scale),
            }
        }
    }

    fn intersects_line(&self, a: Vec2D, b: Vec2D) -> Option<IntersectionResponse> {
        let mut intersections: Vec<IntersectionResponse> = vec![];

        // get the closest intersection point from the start of the line
        for hitbox in self.hitboxes.iter() {
            if let Some(intersection) = match hitbox {
                Hitbox::Circle(hitbox) => hitbox.intersects_line(a, b),
                Hitbox::Rect(hitbox) => hitbox.intersects_line(a, b),
                Hitbox::Polygon(hitbox) => hitbox.intersects_line(a, b),
                Hitbox::Group(hitbox) => hitbox.intersects_line(a, b),
            } {
                intersections.push(intersection);
            }
        }

        intersections.sort_by(|c, d| {
            geometry::distance_squared(c.point, a).partial_cmp(&geometry::distance_squared(d.point, a)).unwrap()
        });

        intersections.first().cloned()
    }

    fn random_point(&self) -> Vec2D {
        match random_item(&self.hitboxes) {
            Hitbox::Circle(hitbox) => hitbox.random_point(),
            Hitbox::Rect(hitbox) => hitbox.random_point(),
            Hitbox::Polygon(hitbox) => hitbox.random_point(),
            Hitbox::Group(hitbox) => hitbox.random_point(),
        }
    }

    fn as_rectangle(&self) -> RectangleHitbox {
        let mut min = Vec2D::new(f64::MAX, f64::MAX);
        let mut max = Vec2D::new(0.0, 0.0);

        fn update<T: Collidable>(hitbox: &T, min: &mut Vec2D, max: &mut Vec2D) {
            let rect = hitbox.as_rectangle();
            min.x = min.x.min(rect.min.x);
            min.y = min.y.min(rect.min.y);
            max.x = max.x.max(rect.max.x);
            max.y = max.y.max(rect.max.y);
        }

        for hitbox in self.hitboxes.iter() {
            match hitbox {
                Hitbox::Circle(hitbox) => update(hitbox, &mut min, &mut max),
                Hitbox::Rect(hitbox) => update(hitbox, &mut min, &mut max),
                Hitbox::Polygon(hitbox) => update(hitbox, &mut min, &mut max),
                Hitbox::Group(hitbox) => update(hitbox, &mut min, &mut max),
            }
        }

        RectangleHitbox {
            min,
            max
        }
    }

    // TODO Test this function thouroughly cuz idk if it works.
    fn is_vec_inside(&self, vec: Vec2D) -> bool {
        for hitbox in self.hitboxes.iter() {
            match hitbox {
                Hitbox::Circle(hitbox) => if hitbox.is_vec_inside(vec) {return true;},
                Hitbox::Rect(hitbox) => if hitbox.is_vec_inside(vec) {return true;},
                Hitbox::Polygon(hitbox) => if hitbox.is_vec_inside(vec) {return true;},
                Hitbox::Group(hitbox) => if hitbox.is_vec_inside(vec) {return true;},
            }
        }

        false
    }

    fn get_center(&self) -> Vec2D {
        self.as_rectangle().get_center()
    }

    fn panic_unknown_subclass(other: &Hitbox) {
        panic!(
            "Hitbox type GroupHitbox doesn't support this operation with hitbox type {:#?}",
            other
        )}
}
